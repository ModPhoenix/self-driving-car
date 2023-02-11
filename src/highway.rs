use leptos::*;
use web_sys::CanvasRenderingContext2d;

use crate::car::Car;

#[component]
pub fn Highway(cx: Scope) -> impl IntoView {
    let window = window();
    console_log(window);
    let canvas: NodeRef<HtmlElement<Canvas>> = NodeRef::new(cx);

    let view = view! { cx,
        <canvas id="canvas" ref=canvas  />
    };

    create_effect(cx, move |_| {
        let canvas = canvas.get().unwrap();
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
        canvas.set_width(200);
        let ctx = canvas.get_context("2d").unwrap().unwrap();
        let ctx = ctx.unchecked_into::<CanvasRenderingContext2d>();

        let car = Car::new(100.0, 100.0, 30.0, 50.0, "red".to_string());

        car.draw(&ctx);
    });

    view
}
