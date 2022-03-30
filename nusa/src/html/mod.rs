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
            attributes: attributes,
            events: events.into(),
            children,
            index_id,
        })
    }
}

macro_rules! element {
    ($tag_name:tt as $f_name:ident) => {
        pub fn $f_name(
            attrs: html_element::Attributes,
            events: html_element::Events,
            children: Vec<Self>,
        ) -> Self {
            Self::element($tag_name, attrs, events, children)
        }
    };
}

impl Html {
    element!("a" as a);
    element!("abbr" as abbr);
    element!("address" as address);
    element!("area" as area);
    element!("article" as article);
    element!("aside" as aside);
    element!("audio" as audio);
    element!("b" as b);
    element!("bdi" as bdi);
    element!("bdo" as bdo);
    element!("blockquote" as blockquote);
    element!("button" as button);
    element!("br" as br);
    element!("cite" as cite);
    element!("caption" as caption);
    element!("canvas" as canvas);
    element!("code" as code);
    element!("col" as col);
    element!("colgroup" as colgroup);
    element!("datalist" as datalist);
    element!("details" as details);
    element!("dd" as dd);
    element!("dfn" as dfn);
    element!("div" as div);
    element!("data" as data);
    element!("del" as del);
    element!("dl" as dl);
    element!("dt" as dt);
    element!("em" as em);
    element!("embed" as embed);
    element!("fieldset" as fieldset);
    element!("figcaption" as figcaption);
    element!("figure" as figure);
    element!("footer" as footer);
    element!("form" as form);
    element!("h1" as h1);
    element!("h2" as h2);
    element!("h3" as h3);
    element!("h4" as h4);
    element!("h5" as h5);
    element!("h6" as h6);
    element!("header" as header);
    element!("hr" as hr);
    element!("i" as i);
    element!("iframe" as iframe);
    element!("img" as img);
    element!("input" as input);
    element!("ins" as ins);
    element!("kbd" as kbd);
    element!("label" as label);
    element!("legend" as legend);
    element!("li" as li);
    element!("main" as main);
    element!("mark" as mark);
    element!("map" as map);
    element!("menu" as menu);
    element!("menuitem" as menuitem);
    element!("meter" as meter);
    element!("nav" as nav);
    element!("object" as object);
    element!("ol" as ol);
    element!("optgroup" as optgroup);
    element!("option" as option);
    element!("output" as output);
    element!("p" as p);
    element!("param" as param);
    element!("picture" as picture);
    element!("pre" as pre);
    element!("progress" as progress);
    element!("q" as q);
    element!("rp" as rp);
    element!("rt" as rt);
    element!("rtc" as rtc);
    element!("rubu" as rubu);
    element!("s" as s);
    element!("samp" as samp);
    element!("section" as section);
    element!("select" as select);
    element!("small" as small);
    element!("source" as source);
    element!("span" as span);
    element!("strong" as strong);
    element!("sub" as sub);
    element!("summary" as summary);
    element!("sup" as sup);
    element!("table" as table);
    element!("tbody" as tbody);
    element!("td" as td);
    element!("textarea" as textarea);
    element!("tfoot" as tfoot);
    element!("th" as th);
    element!("thead" as thead);
    element!("time" as time);
    element!("tr" as tr);
    element!("track" as track);
    element!("u" as u);
    element!("ul" as ul);
    element!("var" as var);
    element!("video" as video);
    element!("wbr" as wbr);
}
