use super::*;
use crate::syntax::{HeadingNode, RawNode};

/// `linebreak`: Start a new line.
///
/// # Syntax
/// This function has dedicated syntax:
/// ```typst
/// This line ends here, \
/// And a new one begins.
/// ```
///
/// # Return value
/// A template that inserts a line break.
pub fn linebreak(_: &mut EvalContext, _: &mut FuncArgs) -> Value {
    Value::template(Node::LINEBREAK, move |ctx| {
        ctx.linebreak();
    })
}

/// `parbreak`: Start a new paragraph.
///
/// # Return value
/// A template that inserts a paragraph break.
pub fn parbreak(_: &mut EvalContext, _: &mut FuncArgs) -> Value {
    Value::template(Node::PARBREAK, move |ctx| {
        ctx.parbreak();
    })
}

/// `strong`: Strong text.
///
/// # Syntax
/// This function has dedicated syntax.
/// ```typst
/// This is *important*!
/// ```
///
/// # Positional parameters
/// - Body: optional, of type `template`.
///
/// # Return value
/// A template that flips the boldness of text. The effect is scoped to the
/// body if present.
pub fn strong(ctx: &mut EvalContext, args: &mut FuncArgs) -> Value {
    let body = args.eat::<TemplateValue>(ctx);
    Value::template(Node::STRONG, move |ctx| {
        let snapshot = ctx.state.clone();
        ctx.state.font.strong ^= true;

        if let Some(body) = &body {
            body.exec(ctx);
            ctx.state = snapshot;
        }
    })
}

/// `emph`: Emphasized text.
///
/// # Syntax
/// This function has dedicated syntax.
/// ```typst
/// I would have _never_ thought so!
/// ```
///
/// # Positional parameters
/// - Body: optional, of type `template`.
///
/// # Return value
/// A template that flips whether text is set in italics. The effect is scoped
/// to the body if present.
pub fn emph(ctx: &mut EvalContext, args: &mut FuncArgs) -> Value {
    let body = args.eat::<TemplateValue>(ctx);
    Value::template(Node::EMPH, move |ctx| {
        let snapshot = ctx.state.clone();
        ctx.state.font.emph ^= true;

        if let Some(body) = &body {
            body.exec(ctx);
            ctx.state = snapshot;
        }
    })
}

/// `heading`: A section heading.
///
/// # Syntax
/// This function has dedicated syntax.
/// ```typst
/// = Section
/// ...
///
/// == Subsection
/// ...
/// ```
///
/// # Positional parameters
/// - Body, of type `template`.
///
/// # Named parameters
/// - Section depth: `level`, of type `integer` between 1 and 6.
///
/// # Return value
/// A template that sets the body as a section heading, that is, large and in
/// bold.
pub fn heading(ctx: &mut EvalContext, args: &mut FuncArgs) -> Value {
    let level = args.eat_named(ctx, HeadingNode::LEVEL).unwrap_or(1);
    let body = args
        .eat_expect::<TemplateValue>(ctx, HeadingNode::BODY)
        .unwrap_or_default();

    Value::template(Node::HEADING, move |ctx| {
        let snapshot = ctx.state.clone();
        let upscale = 1.6 - 0.1 * level as f64;
        ctx.state.font.scale *= upscale;
        ctx.state.font.strong = true;

        body.exec(ctx);
        ctx.state = snapshot;

        ctx.parbreak();
    })
}

/// `raw`: Raw text.
///
/// # Syntax
/// This function has dedicated syntax:
/// - For inline-level raw text:
///   ```typst
///   `...`
///   ```
/// - For block-level raw text:
///   ````typst
///   ```rust
///   println!("Hello World!");
///   ```
///   ````
///
/// # Positional parameters
/// - Text, of type `string`.
///
/// # Named parameters
/// - Language for syntax highlighting: `lang`, of type `string`.
/// - Whether the item is block level (split in its own paragraph): `block`, of
///   type `boolean`.
///
/// # Return value
/// A template that sets the text raw, that is, in monospace and optionally with
/// syntax highlighting.
pub fn raw(ctx: &mut EvalContext, args: &mut FuncArgs) -> Value {
    let text = args.eat_expect::<String>(ctx, RawNode::TEXT).unwrap_or_default();
    let _lang = args.eat_named::<String>(ctx, RawNode::LANG);
    let block = args.eat_named(ctx, RawNode::BLOCK).unwrap_or(false);

    Value::template(Node::RAW, move |ctx| {
        if block {
            ctx.parbreak();
        }

        let snapshot = ctx.state.clone();
        ctx.set_monospace();
        ctx.push_text(text.clone());
        ctx.state = snapshot;

        if block {
            ctx.parbreak();
        }
    })
}
