ace.define(
  'ace/theme/palenight',
  ['require', 'exports', 'module', 'ace/lib/dom'],
  function (e, t, n) {
    ;(t.isDark = !0),
      (t.cssClass = 'ace-palenight'),
      (t.cssText =` 
.ace-palenight .ace_gutter {
  background: var(--bg-editor);
  color: var(--comment);
}

.ace-palenight .ace_print-margin {
  width: 1px;
  background: var(--bg-editor);
}
.ace-palenight {
  background-color: var(--bg-editor);
  color: var(--fg-editor);
}
.ace-palenight .ace_cursor {
  color: var(--white);
}
.ace-palenight .ace_marker-layer .ace_selection {
  background: var(--bg-hover);
}
.ace-palenight.ace_multiselect .ace_selection.ace_start {
  box-shadow: 0 0 3px 0px #1d1f21;
}
.ace-palenight .ace_marker-layer .ace_step {
  background: var(--bg-editor);
}
.ace-palenight .ace_marker-layer .ace_bracket {
  margin: -1px 0 0 -1px;
  border: 1px solid var(--blue);
}
.ace-palenight .ace_marker-layer .ace_active-line {
  background: #282a2e;
}
.ace-palenight .ace_gutter-active-line {
  background-color: #282a2e;
}
.ace-palenight .ace_marker-layer .ace_selected-word {
  border: 1px solid #373b41;
}
.ace-palenight .ace_invisible {
  color: #4b4e55;
}
.ace-palenight .ace_keyword,
.ace-palenight .ace_meta,
.ace-palenight .ace_storage,
.ace-palenight .ace_storage.ace_type,
.ace-palenight .ace_support.ace_type {
  color: var(--magenta);
}
.ace-palenight .ace_keyword.ace_operator {
  color: var(--blue-bright);
}
.ace-palenight .ace_constant.ace_character,
.ace-palenight .ace_constant.ace_language,
.ace-palenight .ace_constant.ace_numeric,
.ace-palenight .ace_keyword.ace_other.ace_unit,
.ace-palenight .ace_support.ace_constant,
.ace-palenight .ace_variable.ace_parameter {
  color: var(--orange);
}
.ace-palenight .ace_constant.ace_other {
  color: #ced1cf;
}
.ace-palenight .ace_invalid {
  color: #ced2cf;
  background-color: #df5f5f;
}
.ace-palenight .ace_invalid.ace_deprecated {
  color: #ced2cf;
  background-color: #b798bf;
}
.ace-palenight .ace_fold {
  background-color: var(--comment);
  border-color: var(--comment);
}
.ace-palenight .ace_entity.ace_name.ace_function,
.ace-palenight .ace_support.ace_function,
.ace-palenight .ace_variable {
  color: var(--blue);
}
.ace-palenight .ace_support.ace_class,
.ace-palenight .ace_support.ace_type {
  color: var(--yellow);
}
.ace-palenight .ace_heading,
.ace-palenight .ace_markup.ace_heading,
.ace-palenight .ace_string {
  color: var(--green);
}
.ace-palenight .ace_entity.ace_name.ace_tag,
.ace-palenight .ace_entity.ace_other.ace_attribute-name,
.ace-palenight .ace_meta.ace_tag,
.ace-palenight .ace_string.ace_regexp,
.ace-palenight .ace_variable {
  color: var(--orange);
}
.ace-palenight .ace_comment {
  color: var(--comment);
}
.ace-palenight .ace_indent-guide {
  background: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAACCAYAAACZgbYnAAAAEklEQVQImWNgYGBgYHB3d/8PAAOIAdULw8qMAAAAAElFTkSuQmCC)
    right repeat-y;
}
        `)
        
    var r = e('../lib/dom')
    r.importCssString(t.cssText, t.cssClass)
  },
)
;(function () {
  ace.require(['ace/theme/palenight'], function (m) {
    if (typeof module == 'object' && typeof exports == 'object' && module) {
      module.exports = m
    }
  })
})()
