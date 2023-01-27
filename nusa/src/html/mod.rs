use crate::HtmlPrefab;

pub mod html_element;
pub mod html_text;

pub use html_element::HtmlElement;
pub use html_text::HtmlText;

pub enum Html {
    Component(Box<dyn HtmlPrefab>),
    HtmlElement(HtmlElement),
    HtmlText(HtmlText),
    Fragment(Vec<Html>),
    RNode(web_sys::Node),
    None,
}

impl Html {
    pub fn text(text: impl Into<String>) -> Self {
        Self::HtmlText(HtmlText { text: text.into() })
    }

    pub fn element(
        tag_name: impl Into<String>,
        attrs: html_element::Attributes,
        events: html_element::Events,
        children: Vec<Self>,
    ) -> Self {
        let (index_id, attributes) = attrs.into_attributes();
        Self::HtmlElement(HtmlElement {
            tag_name: tag_name.into(),
            namespace_name: None,
            attributes: attributes,
            events: events.into(),
            children,
            index_id,
        })
    }

    pub fn element_ns(
        tag_name: impl Into<String>,
        namespace_name: impl Into<String>,
        attrs: html_element::Attributes,
        events: html_element::Events,
        children: Vec<Self>,
    ) -> Self {
        let (index_id, attributes) = attrs.into_attributes();
        Self::HtmlElement(HtmlElement {
            tag_name: tag_name.into(),
            namespace_name: Some(namespace_name.into()),
            attributes: attributes,
            events: events.into(),
            children,
            index_id,
        })
    }

    pub fn none() -> Self {
        Self::None
    }

    pub fn fragment(htmls: Vec<Html>) -> Self {
        Self::Fragment(htmls)
    }

    pub fn node(node: web_sys::Node) -> Self {
        Self::RNode(node)
    }
}

macro_rules! element {
    ($tag_name:tt as $f_name:ident, $f_ns_name:ident) => {
        pub fn $f_name(
            attrs: html_element::Attributes,
            events: html_element::Events,
            children: Vec<Self>,
        ) -> Self {
            Self::element($tag_name, attrs, events, children)
        }

        pub fn $f_ns_name(
            namespace_name: impl Into<String>,
            attrs: html_element::Attributes,
            events: html_element::Events,
            children: Vec<Self>,
        ) -> Self {
            Self::element_ns($tag_name, namespace_name, attrs, events, children)
        }
    };
}

impl Html {
    element!("a" as a, a_ns);
    element!("abbr" as abbr, abbr_ns);
    element!("address" as address, address_ns);
    element!("area" as area, area_ns);
    element!("article" as article, article_ns);
    element!("aside" as aside, aside_ns);
    element!("audio" as audio, audio_ns);
    element!("b" as b, b_ns);
    element!("bdi" as bdi, bdi_ns);
    element!("bdo" as bdo, bdo_ns);
    element!("blockquote" as blockquote, blockquote_ns);
    element!("button" as button, button_ns);
    element!("br" as br, br_ns);
    element!("cite" as cite, cite_ns);
    element!("caption" as caption, caption_ns);
    element!("canvas" as canvas, canvas_ns);
    element!("code" as code, code_ns);
    element!("col" as col, col_ns);
    element!("colgroup" as colgroup, colgroup_ns);
    element!("datalist" as datalist, datalist_ns);
    element!("details" as details, details_ns);
    element!("dd" as dd, dd_ns);
    element!("dfn" as dfn, dfn_ns);
    element!("div" as div, div_ns);
    element!("data" as data, data_ns);
    element!("del" as del, del_ns);
    element!("dl" as dl, dl_ns);
    element!("dt" as dt, dt_ns);
    element!("em" as em, em_ns);
    element!("embed" as embed, embed_ns);
    element!("fieldset" as fieldset, fieldset_ns);
    element!("figcaption" as figcaption, figcaption_ns);
    element!("figure" as figure, figure_ns);
    element!("footer" as footer, footer_ns);
    element!("form" as form, form_ns);
    element!("h1" as h1, h1_ns);
    element!("h2" as h2, h2_ns);
    element!("h3" as h3, h3_ns);
    element!("h4" as h4, h4_ns);
    element!("h5" as h5, h5_ns);
    element!("h6" as h6, h6_ns);
    element!("header" as header, header_ns);
    element!("hr" as hr, hr_ns);
    element!("i" as i, i_ns);
    element!("iframe" as iframe, iframe_ns);
    element!("img" as img, img_ns);
    element!("input" as input, input_ns);
    element!("ins" as ins, ins_ns);
    element!("kbd" as kbd, kbd_ns);
    element!("label" as label, label_ns);
    element!("legend" as legend, legend_ns);
    element!("li" as li, li_ns);
    element!("main" as main, main_ns);
    element!("mark" as mark, mark_ns);
    element!("map" as map, map_ns);
    element!("menu" as menu, menu_ns);
    element!("menuitem" as menuitem, menuitem_ns);
    element!("meter" as meter, meter_ns);
    element!("nav" as nav, nav_ns);
    element!("object" as object, object_ns);
    element!("ol" as ol, ol_ns);
    element!("optgroup" as optgroup, optgroup_ns);
    element!("option" as option, option_ns);
    element!("output" as output, output_ns);
    element!("p" as p, p_ns);
    element!("param" as param, param_ns);
    element!("picture" as picture, picture_ns);
    element!("pre" as pre, pre_ns);
    element!("progress" as progress, progress_ns);
    element!("q" as q, q_ns);
    element!("rp" as rp, rp_ns);
    element!("rt" as rt, rt_ns);
    element!("rtc" as rtc, rtc_ns);
    element!("ruby" as ruby, ruby_ns);
    element!("s" as s, s_ns);
    element!("samp" as samp, samp_ns);
    element!("section" as section, section_ns);
    element!("select" as select, select_ns);
    element!("small" as small, small_ns);
    element!("source" as source, source_ns);
    element!("span" as span, span_ns);
    element!("strong" as strong, strong_ns);
    element!("sub" as sub, sub_ns);
    element!("summary" as summary, summary_ns);
    element!("sup" as sup, sup_ns);
    element!("table" as table, table_ns);
    element!("tbody" as tbody, tbody_ns);
    element!("td" as td, td_ns);
    element!("textarea" as textarea, textarea_ns);
    element!("tfoot" as tfoot, tfoot_ns);
    element!("th" as th, th_ns);
    element!("thead" as thead, thead_ns);
    element!("time" as time, time_ns);
    element!("tr" as tr, tr_ns);
    element!("track" as track, track_ns);
    element!("u" as u, u_ns);
    element!("ul" as ul, ul_ns);
    element!("var" as var, var_ns);
    element!("video" as video, video_ns);
    element!("wbr" as wbr, wbr_ns);
}

impl std::default::Default for Html {
    fn default() -> Self {
        Self::none()
    }
}
