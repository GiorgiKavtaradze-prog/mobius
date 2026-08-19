use std::{collections::BTreeSet, io};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseEvent,
        MouseEventKind,
    },
    execute,
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style},
    symbols,
    text::{Line as TextLine, Span},
    widgets::{
        Block, Clear, Paragraph, Widget,
        canvas::{Canvas, Points},
    },
};
use ratatui_mobius::{MobiusGraphic, MobiusGraphicSettings, ObjectFormat};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = DrawingApp::new()?;
    execute!(io::stdout(), EnableMouseCapture)?;
    while !app.should_quit {
        terminal.draw(|frame| app.render(frame))?;
        app.handle_event()?;
    }
    execute!(io::stdout(), DisableMouseCapture)?;
    app.preview.clear()?;
    Ok(())
}

struct DrawingApp<'a> {
    should_quit: bool,
    canvas_area: Rect,
    preview_area: Rect,
    mouse_position: Option<Position>,
    last_draw_position: Option<Position>,
    last_rotate_position: Option<Position>,
    points: BTreeSet<(u16, u16)>,
    preview: MobiusGraphic<'a>,
}

impl<'a> DrawingApp<'a> {
    fn new() -> io::Result<Self> {
        let preview = MobiusGraphic::new(
            MobiusGraphicSettings::new("live_draw.obj")
                .id(700)
                .format(ObjectFormat::Obj)
                .animate(true)
                .scale(0.6)
                .depth(8.0)
                .color([255, 96, 96]),
        );

        Ok(Self {
            should_quit: false,
            canvas_area: Rect::default(),
            preview_area: Rect::default(),
            mouse_position: None,
            last_draw_position: None,
            last_rotate_position: None,
            points: BTreeSet::new(),
            preview,
        })
    }

    fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) => self.on_key(key)?,
            Event::Mouse(mouse) => self.on_mouse(mouse)?,
            _ => {}
        }
        Ok(())
    }

    fn on_key(&mut self, key: KeyEvent) -> io::Result<()> {
        if !key.is_press() {
            return Ok(());
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('a') => {
                let animate = self.preview.settings().animate;
                self.preview.settings_mut().animate = !animate;
                if !self.points.is_empty() {
                    self.preview.update()?;
                }
            }
            KeyCode::Char('c') => {
                self.points.clear();
                self.last_draw_position = None;
                self.last_rotate_position = None;
                self.preview.clear()?;
            }
            _ => {}
        }
        Ok(())
    }

    fn on_mouse(&mut self, event: MouseEvent) -> io::Result<()> {
        let position = Position::new(event.column, event.row);
        self.mouse_position = Some(position);

        let preview_contains = position.x >= self.preview_area.x
            && position.x < self.preview_area.x.saturating_add(self.preview_area.width)
            && position.y >= self.preview_area.y
            && position.y < self.preview_area.y.saturating_add(self.preview_area.height);
        if preview_contains {
            return self.on_preview_mouse(event, position);
        }

        let Some(local) = self.local_canvas_position(position) else {
            return Ok(());
        };

        match event.kind {
            MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                self.points.insert((local.x, local.y));
                self.last_draw_position = Some(local);
                self.sync_preview()?;
            }
            MouseEventKind::Down(crossterm::event::MouseButton::Right) => {
                self.points.remove(&(local.x, local.y));
                self.last_draw_position = Some(local);
                self.sync_preview()?;
            }
            MouseEventKind::Drag(crossterm::event::MouseButton::Left) => {
                self.draw_line(local)?;
            }
            MouseEventKind::Drag(crossterm::event::MouseButton::Right) => {
                self.erase_line(local)?;
            }
            MouseEventKind::Up(_) => {
                self.last_draw_position = None;
                self.last_rotate_position = None;
            }
            _ => {}
        }
        Ok(())
    }

    fn on_preview_mouse(&mut self, event: MouseEvent, position: Position) -> io::Result<()> {
        match event.kind {
            MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
                self.last_rotate_position = Some(position);
            }
            MouseEventKind::Drag(crossterm::event::MouseButton::Left) => {
                let Some(previous) = self.last_rotate_position else {
                    self.last_rotate_position = Some(position);
                    return Ok(());
                };
                let dx = position.x as i16 - previous.x as i16;
                let dy = position.y as i16 - previous.y as i16;
                self.preview.settings_mut().rotation[1] += f32::from(dx) * 4.0;
                self.preview.settings_mut().rotation[0] += f32::from(dy) * 4.0;
                self.last_rotate_position = Some(position);
                if !self.points.is_empty() {
                    self.preview.update()?;
                }
            }
            MouseEventKind::Up(crossterm::event::MouseButton::Left) => {
                self.last_rotate_position = None;
            }
            _ => {}
        }
        Ok(())
    }

    fn draw_line(&mut self, end: Position) -> io::Result<()> {
        let Some(start) = self.last_draw_position else {
            self.points.insert((end.x, end.y));
            self.last_draw_position = Some(end);
            self.sync_preview()?;
            return Ok(());
        };

        let (mut x0, mut y0) = (i32::from(start.x), i32::from(start.y));
        let (x1, y1) = (i32::from(end.x), i32::from(end.y));
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.points.insert((x0 as u16, y0 as u16));
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }

        self.last_draw_position = Some(end);
        self.sync_preview()
    }

    fn erase_line(&mut self, end: Position) -> io::Result<()> {
        let Some(start) = self.last_draw_position else {
            self.points.remove(&(end.x, end.y));
            self.last_draw_position = Some(end);
            self.sync_preview()?;
            return Ok(());
        };

        let (mut x0, mut y0) = (i32::from(start.x), i32::from(start.y));
        let (x1, y1) = (i32::from(end.x), i32::from(end.y));
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.points.remove(&(x0 as u16, y0 as u16));
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }

        self.last_draw_position = Some(end);
        self.sync_preview()
    }

    fn sync_preview(&mut self) -> io::Result<()> {
        if self.points.is_empty() {
            return self.preview.clear();
        }

        let obj = write_obj(&self.points);
        self.preview.register_payload(obj.as_bytes())
    }

    fn render(&mut self, frame: &mut Frame<'_>) {
        let area = frame.area();
        let header = Rect::new(area.x, area.y, area.width, 3);
        let body = Rect::new(
            area.x,
            area.y.saturating_add(3),
            area.width,
            area.height.saturating_sub(3),
        );
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(body);

        Paragraph::new(TextLine::from(vec![
            Span::styled("left mouse", Style::default().fg(Color::Cyan)),
            Span::raw(": draw  "),
            Span::styled("click & drag", Style::default().fg(Color::Cyan)),
            Span::raw(": rotate  "),
            Span::styled("right mouse", Style::default().fg(Color::Cyan)),
            Span::raw(": erase  "),
            Span::styled("a", Style::default().fg(Color::Cyan)),
            Span::raw(format!(
                ": animate ({})  ",
                u8::from(self.preview.settings().animate)
            )),
            Span::styled("c", Style::default().fg(Color::Cyan)),
            Span::raw(": clear  "),
            Span::styled("q", Style::default().fg(Color::Cyan)),
            Span::raw(": quit"),
        ]))
        .block(Block::bordered().title(Span::styled(
            "Mobius Drawing Demo",
            Style::default().fg(Color::Yellow),
        )))
        .render(header, frame.buffer_mut());

        self.render_canvas(frame, panes[0]);
        self.render_preview(frame, panes[1]);
    }

    fn render_canvas(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered()
            .border_style(Style::default().fg(Color::White))
            .title("Canvas");
        let inner = block.inner(area);
        self.canvas_area = inner;
        let x_max = inner.width.saturating_sub(1);
        let y_max = inner.height.saturating_sub(1);
        let drawn_points: Vec<(f64, f64)> = self
            .points
            .iter()
            .map(|&(x, y)| (f64::from(x), f64::from(y_max.saturating_sub(y))))
            .collect();

        frame.render_widget(
            Canvas::default()
                .block(block)
                .x_bounds([0.0, f64::from(x_max)])
                .y_bounds([0.0, f64::from(y_max)])
                .marker(symbols::Marker::Block)
                .paint(|ctx| {
                    if !drawn_points.is_empty() {
                        ctx.draw(&Points {
                            coords: &drawn_points,
                            color: Color::LightRed,
                        });
                    }
                }),
            area,
        );

        for y in 0..inner.height {
            for x in 0..inner.width {
                if self.points.contains(&(x, y)) {
                    continue;
                }
                if let Some(cell) = frame
                    .buffer_mut()
                    .cell_mut((inner.x.saturating_add(x), inner.y.saturating_add(y)))
                {
                    cell.set_char('·')
                        .set_style(Style::default().fg(Color::Gray));
                }
            }
        }

        if self.points.is_empty() {
            let placeholder = Rect::new(
                inner.x,
                inner.y.saturating_add(inner.height.saturating_sub(1) / 2),
                inner.width,
                1,
            );
            frame.render_widget(Paragraph::new("Draw here!").centered(), placeholder);
        }

        if let Some(position) = self.mouse_position
            && self.local_canvas_position(position).is_some()
        {
            frame.set_cursor_position(position);
        }
    }

    fn render_preview(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::bordered()
            .border_style(Style::default().fg(Color::White))
            .title("Preview");
        let inner = block.inner(area);
        self.preview_area = inner;
        block.render(area, frame.buffer_mut());
        frame.render_widget(Clear, inner);

        if self.points.is_empty() {
            return;
        }

        (&self.preview).render(inner, frame.buffer_mut());
    }

    fn local_canvas_position(&self, position: Position) -> Option<Position> {
        let within_x = position.x >= self.canvas_area.x
            && position.x < self.canvas_area.x.saturating_add(self.canvas_area.width);
        let within_y = position.y >= self.canvas_area.y
            && position.y < self.canvas_area.y.saturating_add(self.canvas_area.height);
        if !within_x || !within_y {
            return None;
        }

        Some(Position::new(
            position.x.saturating_sub(self.canvas_area.x),
            position.y.saturating_sub(self.canvas_area.y),
        ))
    }
}

fn write_obj(points: &BTreeSet<(u16, u16)>) -> String {
    let mut out = String::new();
    let mut vertex = 1u32;

    for &(x, y) in points {
        let x0 = x as f32;
        let y0 = -(y as f32);
        let x1 = x0 + 1.0;
        let y1 = y0 - 1.0;

        out.push_str(&format!("v {x0} {y0} 0.0\n"));
        out.push_str(&format!("v {x1} {y0} 0.0\n"));
        out.push_str(&format!("v {x1} {y1} 0.0\n"));
        out.push_str(&format!("v {x0} {y1} 0.0\n"));
        out.push_str(&format!("f {0} {1} {2}\n", vertex, vertex + 1, vertex + 2));
        out.push_str(&format!("f {0} {1} {2}\n", vertex, vertex + 2, vertex + 3));
        vertex += 4;
    }

    out
}
