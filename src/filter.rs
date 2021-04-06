use metrics::Label;

#[derive(Debug, Clone)]
pub struct LabelFilter;

impl metrics_tracing_context::LabelFilter for LabelFilter {
    fn should_include_label(&self, label: &Label) -> bool {
        let key = label.key();
        key == "component_name" || key == "component_type" || key == "component_kind"
    }
}
