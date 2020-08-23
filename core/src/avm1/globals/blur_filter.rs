//! flash.filter.BlurFilter object

use crate::avm1::activation::Activation;
use crate::avm1::error::Error;
use crate::avm1::function::{Executable, FunctionObject};
use crate::avm1::object::blur_filter::BlurFilterObject;
use crate::avm1::property::Attribute;
use crate::avm1::{Object, TObject, Value};
use enumset::EnumSet;
use gc_arena::MutationContext;
use quick_xml::events::attributes::Attributes;

pub fn constructor<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let blur_x = args
        .get(0)
        .unwrap_or(&4.into())
        .coerce_to_i32(activation)
        .map(|x| x.max(0).min(255))?;

    let blur_y = args
        .get(1)
        .unwrap_or(&4.into())
        .coerce_to_i32(activation)
        .map(|x| x.max(0).min(255))?;

    let quality = args
        .get(2)
        .unwrap_or(&1.into())
        .coerce_to_i32(activation)
        .map(|x| x.max(0).min(15))?;

    println!("args = {}, {}, {}", blur_x, blur_y, quality);

    let blur_filter = this.as_blur_filter_object().unwrap();

    blur_filter.set_blur_x(activation.context.gc_context, blur_x.into());
    blur_filter.set_blur_y(activation.context.gc_context, blur_y.into());
    blur_filter.set_quality(activation.context.gc_context, quality.into());

    println!("constructor called, bf: {:?}", blur_filter);

    Ok(Value::Undefined)
}

pub fn clone<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let proto = activation.context.avm1.prototypes.blur_filter_constructor;

    let blur_x = this.get("blurX", activation)?;
    let blur_y = this.get("blurY", activation)?;
    let quality = this.get("quality", activation)?;

    let cloned = proto.construct(activation, &[blur_x, blur_y, quality])?;
    Ok(cloned.into())
}

pub fn get_blur_x<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    println!("Get blur x");
    Ok(this.as_blur_filter_object().unwrap().get_blur_x().into())
}

pub fn set_blur_x<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let blur_x = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_i32(activation)
        .map(|x| x.max(0).min(255))?;

    this.as_blur_filter_object()
        .unwrap()
        .set_blur_x(activation.context.gc_context, blur_x);

    Ok(Value::Undefined)
}

pub fn get_blur_y<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(this.as_blur_filter_object().unwrap().get_blur_y().into())
}

pub fn set_blur_y<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let blur_y = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_i32(activation)
        .map(|x| x.max(0).min(255))?;

    this.as_blur_filter_object()
        .unwrap()
        .set_blur_y(activation.context.gc_context, blur_y);

    Ok(Value::Undefined)
}

pub fn get_quality<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(this.as_blur_filter_object().unwrap().get_quality().into())
}

pub fn set_quality<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let blur_y = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_i32(activation)
        .map(|x| x.max(0).min(15))?;

    this.as_blur_filter_object()
        .unwrap()
        .set_quality(activation.context.gc_context, blur_y);

    Ok(Value::Undefined)
}

pub fn create_proto<'gc>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc>,
    fn_proto: Object<'gc>,
) -> Object<'gc> {
    let blur_filter = BlurFilterObject::empty_object(gc_context, Some(proto));
    let mut object = blur_filter.as_script_object().unwrap();

    object.force_set_function("clone", clone, gc_context, EnumSet::empty(), Some(fn_proto));

    //TODO: check attribs
    object.add_property(
        gc_context,
        "blurX",
        FunctionObject::function(
            gc_context,
            Executable::Native(get_blur_x),
            Some(fn_proto),
            fn_proto,
        ),
        Some(FunctionObject::function(
            gc_context,
            Executable::Native(set_blur_x),
            Some(fn_proto),
            fn_proto,
        )),
        EnumSet::empty(),
    );

    object.add_property(
        gc_context,
        "blurY",
        FunctionObject::function(
            gc_context,
            Executable::Native(get_blur_y),
            Some(fn_proto),
            fn_proto,
        ),
        Some(FunctionObject::function(
            gc_context,
            Executable::Native(set_blur_y),
            Some(fn_proto),
            fn_proto,
        )),
        EnumSet::empty(),
    );

    object.add_property(
        gc_context,
        "quality",
        FunctionObject::function(
            gc_context,
            Executable::Native(get_quality),
            Some(fn_proto),
            fn_proto,
        ),
        Some(FunctionObject::function(
            gc_context,
            Executable::Native(set_quality),
            Some(fn_proto),
            fn_proto,
        )),
        EnumSet::empty(),
    );

    blur_filter.into()
}