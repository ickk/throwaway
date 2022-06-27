#![allow(dead_code)]

use calamine::{open_workbook, Xlsx, Reader, DataType};
use indoc::indoc;

use std::path::Path;
use std::fs::File;
use std::str;
use std::io::{Cursor, Write};

fn main() {
  let path = std::env::current_dir().unwrap().join("input.xlsx");
  println!("input file: {:?}", path);
  let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
  println!("sheet_names: {:?}", workbook.sheet_names());

  // let mut buff = Cursor::new(Vec::new());
  let output_path = std::env::current_dir().unwrap().join("output.html");
  println!("output file: {:?}", output_path);
  let mut buff = File::create(output_path).unwrap();
  // start document
  write!(buff, indoc!{"
    <!doctype html>
    <html lang=\"en\">
    <head>
      <meta charset=\"utf-8\">
      <title>Table</title>

      <link 
        rel=\"stylesheet\"
        type=\"text/css\"
        href=\"https://cdn.datatables.net/1.10.24/css/jquery.dataTables.css\"
      />
      <script
        src=\"https://code.jquery.com/jquery-3.6.0.slim.min.js\"
        integrity=\"sha256-u7e5khyithlIdTpu22PHhENmPcRdFiHRjhAuHcs05RI=\"
        crossorigin=\"anonymous\"
      ></script>
      <script
        type=\"text/javascript\"
        charset=\"utf8\"
        src=\"https://cdn.datatables.net/1.10.24/js/jquery.dataTables.js\"
      ></script>
      <script type=\"text/javascript\">
        $(document).ready( function () {{
  "});
  for sheet in workbook.sheet_names() {
    write!(buff, "$('#table-{}').DataTable( {{
      \"language\": {{
        \"search\": \"Filter:\"
      }},
      \"sDom\": \"frt\",
    }});\n", sheet);
  }
  write!(buff, indoc!{"
        }} );
      </script>
  "});
  buff.write(r###"<style>
/*src: datatables.net */
table.dataTable{width:100%;margin:0 auto;clear:both;border-collapse:separate;border-spacing:0}table.dataTable tfoot th,table.dataTable thead th{font-weight:bold}table.dataTable thead td,table.dataTable thead th{padding:10px 18px;border-bottom:1px solid #111111}table.dataTable thead td:active,table.dataTable thead th:active{outline:none}table.dataTable tfoot td,table.dataTable tfoot th{padding:10px 18px 6px;border-top:1px solid #111111}table.dataTable thead .sorting,table.dataTable thead .sorting_asc,table.dataTable thead .sorting_asc_disabled,table.dataTable thead .sorting_desc,table.dataTable thead .sorting_desc_disabled{cursor:pointer;*cursor:hand;background-repeat:no-repeat;background-position:center right}table.dataTable thead .sorting{background-image:url("../images/sort_both.png")}table.dataTable thead .sorting_asc{background-image:url("../images/sort_asc.png") !important}table.dataTable thead .sorting_desc{background-image:url("../images/sort_desc.png") !important}table.dataTable thead .sorting_asc_disabled{background-image:url("../images/sort_asc_disabled.png")}table.dataTable thead .sorting_desc_disabled{background-image:url("../images/sort_desc_disabled.png")}table.dataTable tbody tr{background-color:#ffffff}table.dataTable tbody tr.selected{background-color:#b0bed9}table.dataTable tbody td,table.dataTable tbody th{padding:8px 10px}table.dataTable.display tbody td,table.dataTable.display tbody th,table.dataTable.row-border tbody td,table.dataTable.row-border tbody th{border-top:1px solid #dddddd}table.dataTable.display tbody tr:first-child td,table.dataTable.display tbody tr:first-child th,table.dataTable.row-border tbody tr:first-child td,table.dataTable.row-border tbody tr:first-child th{border-top:none}table.dataTable.cell-border tbody td,table.dataTable.cell-border tbody th{border-top:1px solid #dddddd;border-right:1px solid #dddddd}table.dataTable.cell-border tbody tr td:first-child,table.dataTable.cell-border tbody tr th:first-child{border-left:1px solid #dddddd}table.dataTable.cell-border tbody tr:first-child td,table.dataTable.cell-border tbody tr:first-child th{border-top:none}table.dataTable.display tbody tr.odd,table.dataTable.stripe tbody tr.odd{background-color:#f9f9f9}table.dataTable.display tbody tr.odd.selected,table.dataTable.stripe tbody tr.odd.selected{background-color:#acbad4}table.dataTable.display tbody tr:hover,table.dataTable.hover tbody tr:hover{background-color:#f6f6f6}table.dataTable.display tbody tr:hover.selected,table.dataTable.hover tbody tr:hover.selected{background-color:#aab7d1}table.dataTable.display tbody tr > .sorting_1,table.dataTable.display tbody tr > .sorting_2,table.dataTable.display tbody tr > .sorting_3,table.dataTable.order-column tbody tr > .sorting_1,table.dataTable.order-column tbody tr > .sorting_2,table.dataTable.order-column tbody tr > .sorting_3{background-color:#fafafa}table.dataTable.display tbody tr.selected > .sorting_1,table.dataTable.display tbody tr.selected > .sorting_2,table.dataTable.display tbody tr.selected > .sorting_3,table.dataTable.order-column tbody tr.selected > .sorting_1,table.dataTable.order-column tbody tr.selected > .sorting_2,table.dataTable.order-column tbody tr.selected > .sorting_3{background-color:#acbad5}table.dataTable.display tbody tr.odd > .sorting_1,table.dataTable.order-column.stripe tbody tr.odd > .sorting_1{background-color:#f1f1f1}table.dataTable.display tbody tr.odd > .sorting_2,table.dataTable.order-column.stripe tbody tr.odd > .sorting_2{background-color:#f3f3f3}table.dataTable.display tbody tr.odd > .sorting_3,table.dataTable.order-column.stripe tbody tr.odd > .sorting_3{background-color:whitesmoke}table.dataTable.display tbody tr.odd.selected > .sorting_1,table.dataTable.order-column.stripe tbody tr.odd.selected > .sorting_1{background-color:#a6b4cd}table.dataTable.display tbody tr.odd.selected > .sorting_2,table.dataTable.order-column.stripe tbody tr.odd.selected > .sorting_2{background-color:#a8b5cf}table.dataTable.display tbody tr.odd.selected > .sorting_3,table.dataTable.order-column.stripe tbody tr.odd.selected > .sorting_3{background-color:#a9b7d1}table.dataTable.display tbody tr.even > .sorting_1,table.dataTable.order-column.stripe tbody tr.even > .sorting_1{background-color:#fafafa}table.dataTable.display tbody tr.even > .sorting_2,table.dataTable.order-column.stripe tbody tr.even > .sorting_2{background-color:#fcfcfc}table.dataTable.display tbody tr.even > .sorting_3,table.dataTable.order-column.stripe tbody tr.even > .sorting_3{background-color:#fefefe}table.dataTable.display tbody tr.even.selected > .sorting_1,table.dataTable.order-column.stripe tbody tr.even.selected > .sorting_1{background-color:#acbad5}table.dataTable.display tbody tr.even.selected > .sorting_2,table.dataTable.order-column.stripe tbody tr.even.selected > .sorting_2{background-color:#aebcd6}table.dataTable.display tbody tr.even.selected > .sorting_3,table.dataTable.order-column.stripe tbody tr.even.selected > .sorting_3{background-color:#afbdd8}table.dataTable.display tbody tr:hover > .sorting_1,table.dataTable.order-column.hover tbody tr:hover > .sorting_1{background-color:#eaeaea}table.dataTable.display tbody tr:hover > .sorting_2,table.dataTable.order-column.hover tbody tr:hover > .sorting_2{background-color:#ececec}table.dataTable.display tbody tr:hover > .sorting_3,table.dataTable.order-column.hover tbody tr:hover > .sorting_3{background-color:#efefef}table.dataTable.display tbody tr:hover.selected > .sorting_1,table.dataTable.order-column.hover tbody tr:hover.selected > .sorting_1{background-color:#a2aec7}table.dataTable.display tbody tr:hover.selected > .sorting_2,table.dataTable.order-column.hover tbody tr:hover.selected > .sorting_2{background-color:#a3b0c9}table.dataTable.display tbody tr:hover.selected > .sorting_3,table.dataTable.order-column.hover tbody tr:hover.selected > .sorting_3{background-color:#a5b2cb}table.dataTable.no-footer{border-bottom:1px solid #111111}table.dataTable.nowrap td,table.dataTable.nowrap th{white-space:nowrap}table.dataTable.compact thead td,table.dataTable.compact thead th{padding:4px 17px}table.dataTable.compact tfoot td,table.dataTable.compact tfoot th{padding:4px}table.dataTable.compact tbody td,table.dataTable.compact tbody th{padding:4px}table.dataTable td.dt-left,table.dataTable th.dt-left{text-align:left}table.dataTable td.dataTables_empty,table.dataTable td.dt-center,table.dataTable th.dt-center{text-align:center}table.dataTable td.dt-right,table.dataTable th.dt-right{text-align:right}table.dataTable td.dt-justify,table.dataTable th.dt-justify{text-align:justify}table.dataTable td.dt-nowrap,table.dataTable th.dt-nowrap{white-space:nowrap}table.dataTable tfoot td.dt-head-left,table.dataTable tfoot th.dt-head-left,table.dataTable thead td.dt-head-left,table.dataTable thead th.dt-head-left{text-align:left}table.dataTable tfoot td.dt-head-center,table.dataTable tfoot th.dt-head-center,table.dataTable thead td.dt-head-center,table.dataTable thead th.dt-head-center{text-align:center}table.dataTable tfoot td.dt-head-right,table.dataTable tfoot th.dt-head-right,table.dataTable thead td.dt-head-right,table.dataTable thead th.dt-head-right{text-align:right}table.dataTable tfoot td.dt-head-justify,table.dataTable tfoot th.dt-head-justify,table.dataTable thead td.dt-head-justify,table.dataTable thead th.dt-head-justify{text-align:justify}table.dataTable tfoot td.dt-head-nowrap,table.dataTable tfoot th.dt-head-nowrap,table.dataTable thead td.dt-head-nowrap,table.dataTable thead th.dt-head-nowrap{white-space:nowrap}table.dataTable tbody td.dt-body-left,table.dataTable tbody th.dt-body-left{text-align:left}table.dataTable tbody td.dt-body-center,table.dataTable tbody th.dt-body-center{text-align:center}table.dataTable tbody td.dt-body-right,table.dataTable tbody th.dt-body-right{text-align:right}table.dataTable tbody td.dt-body-justify,table.dataTable tbody th.dt-body-justify{text-align:justify}table.dataTable tbody td.dt-body-nowrap,table.dataTable tbody th.dt-body-nowrap{white-space:nowrap}table.dataTable,table.dataTable td,table.dataTable th{box-sizing:content-box}.dataTables_wrapper{position:relative;clear:both;*zoom:1;zoom:1}.dataTables_wrapper .dataTables_length{float:left}.dataTables_wrapper .dataTables_length select{border:1px solid #aaa;border-radius:3px;padding:5px;background-color:transparent;padding:4px}.dataTables_wrapper .dataTables_filter{float:right;text-align:right}.dataTables_wrapper .dataTables_filter input{border:1px solid #aaa;border-radius:3px;padding:5px;background-color:transparent;margin-left:3px}.dataTables_wrapper .dataTables_info{clear:both;float:left;padding-top:0.755em}.dataTables_wrapper .dataTables_paginate{float:right;text-align:right;padding-top:0.25em}.dataTables_wrapper .dataTables_paginate .paginate_button{box-sizing:border-box;display:inline-block;min-width:1.5em;padding:0.5em 1em;margin-left:2px;text-align:center;text-decoration:none !important;cursor:pointer;*cursor:hand;color:#333333 !important;border:1px solid transparent;border-radius:2px}.dataTables_wrapper .dataTables_paginate .paginate_button.current,.dataTables_wrapper .dataTables_paginate .paginate_button.current:hover{color:#333333 !important;border:1px solid #979797;background-color:white;background:-webkit-gradient(linear, left top, left bottom, color-stop(0%, white), color-stop(100%, #dcdcdc));background:-webkit-linear-gradient(top, white 0%, #dcdcdc 100%);background:-moz-linear-gradient(top, white 0%, #dcdcdc 100%);background:-ms-linear-gradient(top, white 0%, #dcdcdc 100%);background:-o-linear-gradient(top, white 0%, #dcdcdc 100%);background:linear-gradient(to bottom, white 0%, #dcdcdc 100%)}.dataTables_wrapper .dataTables_paginate .paginate_button.disabled,.dataTables_wrapper .dataTables_paginate .paginate_button.disabled:active,.dataTables_wrapper .dataTables_paginate .paginate_button.disabled:hover{cursor:default;color:#666 !important;border:1px solid transparent;background:transparent;box-shadow:none}.dataTables_wrapper .dataTables_paginate .paginate_button:hover{color:white !important;border:1px solid #111111;background-color:#585858;background:-webkit-gradient(linear, left top, left bottom, color-stop(0%, #585858), color-stop(100%, #111111));background:-webkit-linear-gradient(top, #585858 0%, #111111 100%);background:-moz-linear-gradient(top, #585858 0%, #111111 100%);background:-ms-linear-gradient(top, #585858 0%, #111111 100%);background:-o-linear-gradient(top, #585858 0%, #111111 100%);background:linear-gradient(to bottom, #585858 0%, #111111 100%)}.dataTables_wrapper .dataTables_paginate .paginate_button:active{outline:none;background-color:#2b2b2b;background:-webkit-gradient(linear, left top, left bottom, color-stop(0%, #2b2b2b), color-stop(100%, #0c0c0c));background:-webkit-linear-gradient(top, #2b2b2b 0%, #0c0c0c 100%);background:-moz-linear-gradient(top, #2b2b2b 0%, #0c0c0c 100%);background:-ms-linear-gradient(top, #2b2b2b 0%, #0c0c0c 100%);background:-o-linear-gradient(top, #2b2b2b 0%, #0c0c0c 100%);background:linear-gradient(to bottom, #2b2b2b 0%, #0c0c0c 100%);box-shadow:inset 0 0 3px #111}.dataTables_wrapper .dataTables_paginate .ellipsis{padding:0 1em}.dataTables_wrapper .dataTables_processing{position:absolute;top:50%;left:50%;width:100%;height:40px;margin-left:-50%;margin-top:-25px;padding-top:20px;text-align:center;font-size:1.2em;background-color:white;background:-webkit-gradient(linear, left top, right top, color-stop(0%, rgba(255, 255, 255, 0)), color-stop(25%, rgba(255, 255, 255, 0.9)), color-stop(75%, rgba(255, 255, 255, 0.9)), color-stop(100%, rgba(255, 255, 255, 0)));background:-webkit-linear-gradient(left, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 0.9) 25%, rgba(255, 255, 255, 0.9) 75%, rgba(255, 255, 255, 0) 100%);background:-moz-linear-gradient(left, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 0.9) 25%, rgba(255, 255, 255, 0.9) 75%, rgba(255, 255, 255, 0) 100%);background:-ms-linear-gradient(left, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 0.9) 25%, rgba(255, 255, 255, 0.9) 75%, rgba(255, 255, 255, 0) 100%);background:-o-linear-gradient(left, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 0.9) 25%, rgba(255, 255, 255, 0.9) 75%, rgba(255, 255, 255, 0) 100%);background:linear-gradient(to right, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 0.9) 25%, rgba(255, 255, 255, 0.9) 75%, rgba(255, 255, 255, 0) 100%)}.dataTables_wrapper .dataTables_filter,.dataTables_wrapper .dataTables_info,.dataTables_wrapper .dataTables_length,.dataTables_wrapper .dataTables_paginate,.dataTables_wrapper .dataTables_processing{color:#333333}.dataTables_wrapper .dataTables_scroll{clear:both}.dataTables_wrapper .dataTables_scroll div.dataTables_scrollBody{*margin-top:-1px;-webkit-overflow-scrolling:touch}.dataTables_wrapper .dataTables_scroll div.dataTables_scrollBody > table > tbody > tr > td,.dataTables_wrapper .dataTables_scroll div.dataTables_scrollBody > table > tbody > tr > th,.dataTables_wrapper .dataTables_scroll div.dataTables_scrollBody > table > thead > tr > td,.dataTables_wrapper .dataTables_scroll div.dataTables_scrollBody > table > thead > tr > th{vertical-align:middle}.dataTables_wrapper .dataTables_scroll div.dataTables_scrollBody > table > tbody > tr > td > div.dataTables_sizing,.dataTables_wrapper .dataTables_scroll div.dataTables_scrollBody > table > tbody > tr > th > div.dataTables_sizing,.dataTables_wrapper .dataTables_scroll div.dataTables_scrollBody > table > thead > tr > td > div.dataTables_sizing,.dataTables_wrapper .dataTables_scroll div.dataTables_scrollBody > table > thead > tr > th > div.dataTables_sizing{height:0;overflow:hidden;margin:0 !important;padding:0 !important}.dataTables_wrapper.no-footer .dataTables_scrollBody{border-bottom:1px solid #111111}.dataTables_wrapper.no-footer div.dataTables_scrollBody > table,.dataTables_wrapper.no-footer div.dataTables_scrollHead table.dataTable{border-bottom:none}.dataTables_wrapper:after{visibility:hidden;display:block;content:"";clear:both;height:0}@media screen and (max-width: 767px){.dataTables_wrapper .dataTables_info,.dataTables_wrapper .dataTables_paginate{float:none;text-align:center}.dataTables_wrapper .dataTables_paginate{margin-top:0.5em}}@media screen and (max-width: 640px){.dataTables_wrapper .dataTables_filter,.dataTables_wrapper .dataTables_length{float:none;text-align:center}.dataTables_wrapper .dataTables_filter{margin-top:0.5em}}
/*src: https://webdevtrick.com/pure-css-tabs-responsive/ */
.tabs{display:flex;flex-wrap:wrap}.tabs>label{order:1;display:block;padding:1rem 2rem;margin-right:0.2rem;cursor:pointer;background:#b6bcc1;font-weight:bold;transition:background ease 0.2s}.tabs .tab{order:99;flex-grow:1;width:100%;display:none;padding:1rem;background:#fff}.tabs input[type="radio"]{display:none}.tabs input[type="radio"]:checked + label{background:#fff}.tabs input[type="radio"]:checked + label + .tab{display:block}@media (max-width: 45em){.tabs .tab,.tabs label{order:initial}.tabs label{width:100%;margin-right:0;margin-top:0.2rem}}body{background:#333;min-height:100vh;box-sizing:border-box;padding-top:10vh;font-family:"HelveticaNeue-Light", "Helvetica Neue Light", "Helvetica Neue", Helvetica, Arial, "Lucida Grande", sans-serif;font-weight:300;line-height:1.5;max-width:60rem;margin:0 auto;font-size:112%}
</style>"###.as_bytes());
  write!(buff, indoc!{"
    </head>
    <body>
    <div class=\"tabs\">
  "});
  // each table
  for (i, (sheet_name, sheet)) in workbook.worksheets().iter().enumerate() {
    let mut rows = sheet.rows();
    let header = match rows.next() {
      Some(val) => val,
      None => continue,
    };
    write!(buff, "\n\
    <input type=\"radio\" name=\"tabs\" id=\"tab-{sheet_name}\" {}>\n\
    <label for=\"tab-{sheet_name}\">{sheet_name}</label>\n\
    <div class=\"tab\">\
    ", match i {0=>"checked=\"checked\"", _=>""}, sheet_name=sheet_name);
    write!(buff, "\n<table id=\"table-{}\" class=\"display compact nowrap\">", sheet_name);
    write!(buff, "\n<thead><tr>");
    for cell in header {
      match cell {
        DataType::String(contents) => write!(buff, "<th>{}</th>", contents),
        _ => write!(buff, "<th>{:?}</th>", cell),
      };
    }
    write!(buff, "</tr></thead>");
    write!(buff, "\n<tbody>");
    for row in rows {
      write!(buff, "\n<tr>");
      for cell in row {
        match cell {
          DataType::String(contents) => write!(buff, "<td>{}</td>", contents),
          DataType::Empty => write!(buff, "<td></td>"),
          _ => write!(buff, "<td>{:?}</td>", cell),
        };
      }
      write!(buff, "</tr>");
    }
    write!(buff, "\n</tbody>");
    write!(buff, "\n</table>");
    write!(buff, "\n</div>\n");
  }
  // end document
  write!(buff, indoc!{"
    </div>
    </body>
    </html>
  "});

  // let result = buff.into_inner();
  // println!("result: \n{}", str::from_utf8(&result).unwrap());
}