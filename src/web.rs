use anyhow::Error;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, MouseEvent};
use yew::format::Nothing;
use yew::services::fetch::{FetchTask, Request, Response, Uri};
use yew::services::{ConsoleService, FetchService};
use yew::{html, Component, Html, NodeRef};

use crate::la::{Matrix, MatrixI, Vec3f};
use crate::model::{self, Wavefront};
use crate::shader::{triangle, BasicShader, LightShader, Shader, ShaderConf};
use crate::tga::Image;
use crate::camera::Camera;
// use crate::transform::{get_prespective_projection};
const WIDTH:u32 = 860;
const HEIGHT:u32 = 512;
pub enum Msg {
    Texture(Vec<u8>),
    Model(Vec<u8>),
    Normals(Vec<u8>),
    Upd(Vec3f),
    UpdC(Vec3f, Vec3f),
    Load(ModelType),
    Diff,
    Spec,
    Txt,
    Zbuff,
    Norm,
    Occl,
    RotationStarted(i32, i32),
    RotationEnded,
    MoveStarted(i32, i32),
    MoveEnded,
    Noop,
}

pub enum ModelType {
    DIABLO,
    AFRICAN,
}

pub struct Model {
    conf: ShaderConf,
    zbuff: bool,
    node_ref: NodeRef,
    props: (),
    link: yew::ComponentLink<Self>,
    task: Vec<Option<FetchTask>>,
    texture: Option<Image>,
    wavefront: Option<Wavefront>,
    normals: Option<Image>,
    model: Option<model::Model>,
    model_type: ModelType,
    camera: Camera,
    rotation_start: Option<(i32, i32, Vec3f)>,
    move_start: Option<(i32, i32, Vec3f)>,
}

impl Model {
    fn render(&mut self) {
        let width: i32 = WIDTH as i32;
        let height: i32 = HEIGHT as i32;
        let mut out_texture = Image::new(width, height);
        let mut z_buffer = Image::new(width, height);
        let mut light_texture = Image::new(width, height);

        let camera = &self.camera;
        let lookat_m = camera.get_lookat_view();
        let lookat_mi = lookat_m.inverse().transpose();
        // let light_dir: Vec3f = get_prespective_projection(5f32).mul(&lookat_m).mul(&Vec3f(1.0, -0.0, 0.5).embed::<4>(1f32)).into();
        let light_dir: Vec3f = Vec3f(1.0, -0.0, 0.5).normalize();
        let model = self.model.as_ref().unwrap();
        let mut shader = BasicShader {
            conf: self.conf.clone(),
            normal_face_vec: None,
            light_dir,
            lookat_m,
            lookat_mi,
            model,
            out_texture: &mut out_texture,
            z_buffer: &mut z_buffer,
            varying_uv: Matrix::zeroed(),
            varying_xy: Matrix::zeroed(),
            vertices: [Vec3f::zeroed(); 3],
            light_texture: &mut light_texture,
        };

        for f in 0..model.num_faces() {
            let mut vertices = [Vec3f::zeroed(), Vec3f::zeroed(), Vec3f::zeroed()];
            for v in 0..3 {
                vertices[v] = shader.vertex(f, v);
            }
            triangle(&vertices[0], &vertices[1], &vertices[2], &mut shader);
        }

        let light_model = model::Model::screen_texture_model();

        if self.conf.occlusion {
            let mut occl_texture = Image::new(width, height);
            let mut light_shader = LightShader {
                conf: ShaderConf::new(),
                model: &light_model,
                out_texture: &mut out_texture,
                light_texture: &mut light_texture,
                z_buffer: &mut z_buffer,
                varying_uv: Matrix::zeroed(),
                varying_xy: Matrix::zeroed(),
                occl_texture: &mut occl_texture,
            };

            for f in 0..light_model.num_faces() {
                let mut vertices = [Vec3f::zeroed(), Vec3f::zeroed(), Vec3f::zeroed()];
                for v in 0..3 {
                    vertices[v] = light_shader.vertex(f, v);
                }
                triangle(&vertices[0], &vertices[1], &vertices[2], &mut light_shader);
            }
        }

        out_texture.apply_gamma(1.5);

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        // canvas.set_width(WIDTH);
        // canvas.set_height(HEIGHT);
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        let img = if self.zbuff { z_buffer } else { out_texture }.get_raw_bytes();
        let id = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&img[..]),WIDTH,HEIGHT).unwrap();
        ctx.put_image_data(&id, 0.0, 0.0).unwrap();
    }

    fn prepare(&mut self) {
        self.model = Some(model::Model::new(
            self.wavefront.take().unwrap(),
            self.normals.take().unwrap(),
            self.texture.take().unwrap(),
        ));
    }

    fn ready(&self) -> bool {
        self.model.is_some()
            || (self.texture.is_some() && self.wavefront.is_some() && self.normals.is_some())
    }

    fn load_binary(&mut self, url: String, dispatch: fn(Vec<u8>) -> Msg) {
        let get_request = Request::get(Uri::builder().path_and_query(url).build().unwrap())
            .body(Nothing)
            .expect("Could not build that request");
        let callback = self
            .link
            .callback(move |response: Response<Result<Vec<u8>, Error>>| {
                let data = response.into_body().unwrap();
                dispatch(data)
            });
        let task =
            FetchService::fetch_binary(get_request, callback).expect("failed to start request");
        self.task.push(Some(task));
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        if self.props != _props {
            self.props = _props;
            true
        } else {
            false
        }
    }

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            zbuff: false,
            conf: ShaderConf::new(),
            task: Vec::new(),
            link,
            props,
            node_ref: NodeRef::default(),
            texture: None,
            wavefront: None,
            normals: None,
            model: None,
            model_type: ModelType::AFRICAN,
            camera: Default::default(),
            rotation_start: None,
            move_start: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.load_binary("./african_head/texture.tga".to_owned(), Msg::Texture);
            self.load_binary("./african_head/normals.tga".to_owned(), Msg::Normals);
            self.load_binary("./african_head/model.obj".to_owned(), Msg::Model);
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Zbuff => {
                self.zbuff = !self.zbuff;
                if self.ready() {
                    self.render();
                }
                true
            }
            Msg::Occl => {
                self.conf = ShaderConf {
                    occlusion: !self.conf.occlusion,
                    ..self.conf
                };
                if self.ready() {
                    self.render();
                }
                true
            }
            Msg::Norm => {
                self.conf = ShaderConf {
                    normals: !self.conf.normals,
                    ..self.conf
                };
                if self.ready() {
                    self.render();
                }
                true
            }
            Msg::Diff => {
                self.conf = ShaderConf {
                    diff_light: !self.conf.diff_light,
                    ..self.conf
                };
                if self.ready() {
                    self.render();
                }
                true
            }
            Msg::Spec => {
                self.conf = ShaderConf {
                    spec_light: !self.conf.spec_light,
                    ..self.conf
                };
                if self.ready() {
                    self.render();
                }
                true
            }
            Msg::Txt => {
                self.conf = ShaderConf {
                    texture: !self.conf.texture,
                    ..self.conf
                };
                if self.ready() {
                    self.render();
                }
                true
            }
            Msg::Upd(v) => {
                self.camera.position = v;
                if self.ready() {
                    self.render();
                }
                true
            }
            Msg::Texture(v) => {
                self.texture = Some(Image::from_raw_vec(v));
                if self.ready() {
                    self.prepare();
                    self.render();
                }
                true
            }
            Msg::Normals(v) => {
                self.normals = Some(Image::from_raw_vec(v));
                if self.ready() {
                    self.prepare();
                    self.render();
                }
                true
            }
            Msg::Model(v) => {
                self.wavefront = Some(Wavefront::parse_string(String::from_utf8(v).unwrap()));
                if self.ready() {
                    self.prepare();
                    self.render();
                }
                true
            }
            Msg::RotationStarted(x, y) => {
                self.rotation_start = Some((x, y, self.camera.position));
                true
            }
            Msg::Noop => false,
            Msg::RotationEnded => {
                self.rotation_start = None;
                true
            }
            Msg::MoveStarted(x, y) => {
                self.move_start = Some((x, y, self.camera.look_at));
                true
            }
            Msg::MoveEnded => {
                self.move_start = None;
                true
            }
            Msg::UpdC(Vec3f(dx, dy, _), old_place) => {
                ConsoleService::log(format!("{:?}, {:?}", dx, dy).as_str());
                let camvec = Vec3f(self.camera.position.0, 0.0, self.camera.position.2)
                    .normalize()
                    .mulf(dy / 500.0);
                let perp: Vec3f = Vec3f(0.0, 1.0, 0.0)
                    .cross(&self.camera.position)
                    .normalize()
                    .mulf(dx / 500.0);

                self.camera.look_at = old_place.add(&perp).add(&camvec);

                if self.ready() {
                    self.render();
                }
                true
            }
            Msg::Load(mt) => {
                match mt {
                    ModelType::AFRICAN => {
                        if let ModelType::AFRICAN = self.model_type {
                        } else {
                            self.model = None;
                            self.texture = None;
                            self.normals = None;
                            self.wavefront = None;
                            self.model_type = ModelType::AFRICAN;
                            self.load_binary("./african_head/texture.tga".to_owned(), |v| {
                                Msg::Texture(v)
                            });
                            self.load_binary("./african_head/normals.tga".to_owned(), |v| {
                                Msg::Normals(v)
                            });
                            self.load_binary("./african_head/model.obj".to_owned(), |v| {
                                Msg::Model(v)
                            });
                        }
                    }
                    ModelType::DIABLO => {
                        if let ModelType::DIABLO = self.model_type {
                        } else {
                            self.model = None;
                            self.texture = None;
                            self.normals = None;
                            self.wavefront = None;
                            self.model_type = ModelType::DIABLO;
                            self.load_binary("./diablo/texture.tga".to_owned(), |v| {
                                Msg::Texture(v)
                            });
                            self.load_binary("./diablo/normals.tga".to_owned(), |v| {
                                Msg::Normals(v)
                            });
                            self.load_binary("./diablo/model.obj".to_owned(), Msg::Model);
                        }
                    }
                }
                false
            }
        }
    }

    fn view(&self) -> Html {
        let Vec3f(x, y, z) = self.camera.position;
        let pos = self.rotation_start;
        let place = self.move_start;
        html! {
            <div class="rusterizer-window"
            oncontextmenu=self.link.callback(move |e: MouseEvent| {
                e.prevent_default();
                Msg::Noop
            })
            onmousedown=self.link.callback(move |e: MouseEvent| {
                if e.button() == 0 {
                    Msg::RotationStarted(e.client_x(), e.client_y())
                } else {
                    Msg::MoveStarted(e.client_x(), e.client_y())
                }
            })
            onmouseup=self.link.callback(move |e: MouseEvent| {
                if e.button() == 0 {
                    Msg::RotationEnded
                } else {
                    Msg::MoveEnded
                }
            })
            onmousemove=self.link.callback(move |e: MouseEvent| {
                if pos.is_some(){
                    pos.map(|(px, py, campos)| {
                        let dx = px - e.client_x();
                        let dy = py - e.client_y();
                        Msg::Upd(campos.rotate(dy as f32/100.0, dx as f32/100.0))
                    }).unwrap_or(Msg::Noop)
                } else {
                    place.map(|(px, py, old_place)| {
                        let dx = px - e.client_x();
                        let dy = py - e.client_y();
                        Msg::UpdC(Vec3f(dx as f32, dy as f32, 0.0), old_place)
                    }).unwrap_or(Msg::Noop)
                }

            })>
                <canvas
                ref={self.node_ref.clone()} width=format!("{}", WIDTH) height=format!("{}", HEIGHT) />
                <div class="menu">
                    { if self.ready() { html! {
                        <>
                            <div class="button-row">
                                <button onclick=self.link.callback(move |_| Msg::Upd(Vec3f(x+0.1, y, z)))>{ "+" }</button>
                                { "x: " }{ format!("{:.2}", x) }
                                <button onclick=self.link.callback(move |_| Msg::Upd(Vec3f(x-0.1, y, z)))>{ "-" }</button>
                            </div>
                            <div class="button-row">
                                <button onclick=self.link.callback(move |_| Msg::Upd(Vec3f(x, y+0.1, z)))>{ "+" }</button>
                                { "y: " }{ format!("{:.2}", y) }
                                <button onclick=self.link.callback(move |_| Msg::Upd(Vec3f(x, y-0.1, z)))>{ "-" }</button>
                            </div>
                            <div class="button-row">
                                <button onclick=self.link.callback(move |_| Msg::Upd(Vec3f(x, y, z+0.1)))>{ "+" }</button>
                                { "z: " }{ format!("{:.2}", z) }
                                <button onclick=self.link.callback(move |_| Msg::Upd(Vec3f(x, y, z-0.1)))>{ "-" }</button>
                            </div>
                            <button class=if self.conf.diff_light { "" } else { "off" } disabled={ self.zbuff } onclick=self.link.callback(move |_| Msg::Diff)>{ "Diffuse light" }</button>
                            <button class=if self.conf.spec_light { "" } else { "off" } disabled={ self.zbuff } onclick=self.link.callback(move |_| Msg::Spec)>{ "Specular light" }</button>
                            <button class=if self.conf.texture { "" } else { "off" } disabled={ self.zbuff } onclick=self.link.callback(move |_| Msg::Txt)>{ "Texture" }</button>
                            <button class=if self.conf.normals { "" } else { "off" } disabled={ self.zbuff } onclick=self.link.callback(move |_| Msg::Norm)>{ "Normal map" }</button>
                            <button class=if self.conf.occlusion { "" } else { "off" } disabled={ self.zbuff } onclick=self.link.callback(move |_| Msg::Occl)>{ "Ambient occlusion" }</button>
                            <button onclick=self.link.callback(move |_| Msg::Zbuff)>{ "Z Buffer" }</button>
                            <div style="height: 100px"></div>
                            <button class=if let ModelType::AFRICAN=self.model_type { "off" } else { "" } onclick=self.link.callback(move |_| Msg::Load(ModelType::AFRICAN))>{ "African head" }</button>
                            <button class=if let ModelType::DIABLO=self.model_type { "off" } else { "" } onclick=self.link.callback(move |_| Msg::Load(ModelType::DIABLO))>{ "Diablo" }</button>
                        </>
                    } } else { html! { "Loading model.." } } }
                </div>
            </div>
        }
    }
}

pub fn web() {
    yew::start_app::<Model>();
}
