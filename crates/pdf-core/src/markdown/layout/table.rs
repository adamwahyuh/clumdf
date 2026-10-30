//! Tabel: kolom sama lebar, teks dibungkus di dalam sel, baris judul tebal berlatar.

use super::lines::{LINE_SPACING, break_lines};
use super::{Ctx, Layouter, Op};
use crate::markdown::blocks::{Span, Style};

const BORDER: [f32; 3] = [0.75, 0.75, 0.75];
const HEADER_BG: [f32; 3] = [0.94, 0.94, 0.93];
const PAD: f32 = 5.0;

pub(super) fn table(l: &mut Layouter, header: &[Vec<Span>], rows: &[Vec<Vec<Span>>], ctx: &Ctx) {
    let size = l.s.font_size * 0.95;
    let columns = rows.iter().map(Vec::len).chain([header.len()]).max().unwrap_or(1).max(1);
    let col_width = ctx.width / columns as f32;
    let bold = |cell: &Vec<Span>| -> Vec<Span> {
        cell.iter().map(|s| Span { style: Style { bold: true, ..s.style.clone() }, ..s.clone() }).collect()
    };
    let header_row: Vec<Vec<Span>> = header.iter().map(bold).collect();

    for (r, row) in std::iter::once(&header_row).chain(rows).enumerate() {
        let cells: Vec<_> = (0..columns)
            .map(|c| break_lines(row.get(c).map_or(&[][..], Vec::as_slice), col_width - 2.0 * PAD, size, l.s.serif))
            .collect();
        let content: f32 = cells.iter().map(|lines| lines.iter().map(|ln| ln.height).sum::<f32>()).fold(0.0, f32::max);
        // shortcut: baris yang lebih tinggi dari satu halaman tidak dipecah; jarang di dokumen biasa
        let row_height = content.max(size * LINE_SPACING) + 2.0 * PAD;
        l.ensure(row_height);

        let (top, bottom) = (l.pdf_y(l.y), l.pdf_y(l.y + row_height));
        if r == 0 {
            l.op(Op::Rect { x: ctx.x, y: bottom, w: ctx.width, h: row_height, color: HEADER_BG });
        }
        for (c, lines) in cells.iter().enumerate() {
            let x = ctx.x + c as f32 * col_width;
            let mut y = l.y + PAD;
            for line in lines {
                l.line(line, x + PAD, y, ctx.color);
                y += line.height;
            }
        }
        // Garis atas, bawah, dan pemisah kolom
        let right = ctx.x + ctx.width;
        l.op(Op::Line { x1: ctx.x, y1: top, x2: right, y2: top, width: 0.5, color: BORDER });
        l.op(Op::Line { x1: ctx.x, y1: bottom, x2: right, y2: bottom, width: 0.5, color: BORDER });
        for c in 0..=columns {
            let x = ctx.x + c as f32 * col_width;
            l.op(Op::Line { x1: x, y1: top, x2: x, y2: bottom, width: 0.5, color: BORDER });
        }
        l.advance(row_height, ctx);
    }
    l.advance(ctx.gap, ctx);
}
