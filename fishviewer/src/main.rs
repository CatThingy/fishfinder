use druid::{
    piet::InterpolationMode,
    widget::{Button, Flex},
    AppLauncher, RenderContext, Size, Widget, WindowDesc,
};
use fishgen::{random_fish, FishOutput};

fn main() {
    let main_window = WindowDesc::new(build_root);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(random_fish(1024, 1024, 0.1, 1.0).unwrap())
        .unwrap();
}

struct FishDisplay;

impl Widget<FishOutput> for FishDisplay {
    fn event(
        &mut self,
        _: &mut druid::EventCtx,
        _: &druid::Event,
        _: &mut FishOutput,
        _: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _: &mut druid::LifeCycleCtx,
        _: &druid::LifeCycle,
        _: &FishOutput,
        _: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        _: &FishOutput,
        _: &FishOutput,
        _: &druid::Env,
    ) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _: &FishOutput,
        _: &druid::Env,
    ) -> druid::Size {
        bc.constrain_aspect_ratio(1.0, 1024.0)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &FishOutput, _: &druid::Env) {
        let size = ctx.size();
        let image = ctx
            .make_image(
                1024,
                1024,
                data.image.as_raw(),
                druid::piet::ImageFormat::Rgb,
            )
            .unwrap();

        let min_square = Size::new(size.min_side(), size.min_side()).to_rect();

        ctx.draw_image(&image, min_square, InterpolationMode::Bilinear);
    }
}

fn build_root() -> impl Widget<FishOutput> {
    // const DARK_GREY: Color = Color::grey8(0x3a);
    Flex::row()
        .with_flex_child(FishDisplay, 2.0)
        .with_flex_child(
            Flex::column().with_flex_child(
                Button::new("Next fish").on_click(|_, data: &mut FishOutput, _| {
                    *data = random_fish(1024, 1024, 0.1, 1.0).unwrap();
                }),
                // Label::new("top left")
                //     .center()
                //     .border(DARK_GREY, 4.0)
                //     .padding(10.0),
                1.0,
            ),
            1.0,
        )
}
