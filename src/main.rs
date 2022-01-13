use web_sys::HtmlInputElement;
use yew::prelude::*;

enum Msg {
    ReadInput,
    GenerateRandomColour,
    DoNothing,
}

struct Page {
    colour: String,
    placeholder: String,
    node_ref: NodeRef,
}

impl Component for Page {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Page {
            colour: "rgb(73, 206, 235)".to_string(),
            placeholder: "red".to_string(),
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReadInput => {
                if let Some(colour) = self.node_ref.cast::<HtmlInputElement>() {
                    if colour.value() == *"" {
                        self.colour = self.placeholder.to_string();
                    } else {
                        self.colour = colour.value();
                    }
                    return true;
                }
                false
            }
            Msg::GenerateRandomColour => {
                if let Ok(rgb) = get_rgb_values() {
                    let rgb_value = format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2]);
                    self.colour = rgb_value;
                    return true;
                }
                false
            }
            Msg::DoNothing => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = {
            ctx.link().callback(|event: KeyboardEvent| {
                if event.key() == "Enter" {
                    Msg::ReadInput
                } else {
                    Msg::DoNothing
                }
            })
        };

        let onclick = { ctx.link().callback(|_| Msg::ReadInput) };

        html!(
            <div class={"centred"}>
                <h1 class={"text"}>{"Colour Generator"}</h1>
                <div class={"svg-box"}>
                    <svg width={"300"} height={"150"}>
                        <rect class={"rect"} width={"100%"} height={"100%"} rx={"30"} fill={self.colour.to_string()}/>
                    </svg>
                    <div class={"text text-rgb"} >{self.colour.to_string()}</div>
                </div>
                <br/>
                <div>
                    <input {onkeypress} placeholder={self.placeholder.to_string()} ref={self.node_ref.clone()}/>
                    <button {onclick}>{"Change Colour"}</button>
                </div><br/>
                <button onclick={ctx.link().callback(|_| Msg::GenerateRandomColour)}>{"Generate Random Colour"}</button>
                <p class={"text"}>
                    <em>{"Colour names, Hex codes and RGB values are all valid, including..."}</em><br/>
                    {"red "}<svg width={"10"} height={"10"}><rect width={"100%"} height={"100%"} rx={"1"} fill={"red".to_string()}/></svg><br/>
                    {"#11859E "}<svg width={"10"} height={"10"}><rect width={"100%"} height={"100%"} rx={"1"} fill={"#11859E".to_string()}/></svg><br/>
                    {"rgb(240, 128, 128) "}<svg width={"10"} height={"10"}><rect width={"100%"} height={"100%"} rx={"1"} fill={"rgb(240, 128, 128)".to_string()}/></svg><br/>
                </p>
            </div>
        )
    }
}

fn main() {
    yew::start_app::<Page>();
}

fn get_rgb_values() -> Result<[u8; 3], getrandom::Error> {
    let mut buf = [0u8; 3];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}
