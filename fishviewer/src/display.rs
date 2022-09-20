use druid::{piet::InterpolationMode, RenderContext, Size, Widget};
use fishgen::FishOutput;

pub struct FishDisplay;

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
                druid::piet::ImageFormat::RgbaSeparate,
            )
            .unwrap();

        let min_square = Size::new(size.min_side(), size.min_side()).to_rect();

        ctx.draw_image(&image, min_square, InterpolationMode::Bilinear);
    }
}
