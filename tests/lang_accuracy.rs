#[path = "common/mod.rs"]
mod common;

fn run_on(fixture: &str) -> serde_json::Value {
    let path = common::lang_dir().join(fixture);
    common::run_json(&path, &["--output", "json", "--no-ignore"])
}

fn snapshots_dir() -> std::path::PathBuf {
    common::lang_dir().join("snapshots")
}

fn load_snapshot(name: &str) -> serde_json::Value {
    common::load_snapshot(&snapshots_dir(), name)
}

#[test]
fn lang_daml_daml() {
    common::assert_snapshot(&run_on("Daml.daml"), &snapshots_dir(), "Daml_daml");
}

#[test]
fn lang_msbuild_csproj() {
    common::assert_snapshot(
        &run_on("MSBuild.csproj"),
        &snapshots_dir(),
        "MSBuild_csproj",
    );
}

#[test]
fn lang_modelica_mo() {
    common::assert_snapshot(&run_on("Modelica.mo"), &snapshots_dir(), "Modelica_mo");
}

#[test]
fn lang_tera_tera() {
    common::assert_snapshot(&run_on("Tera.tera"), &snapshots_dir(), "Tera_tera");
}

#[test]
fn lang_abnf_abnf() {
    common::assert_snapshot(&run_on("abnf.abnf"), &snapshots_dir(), "abnf_abnf");
}

#[test]
fn lang_alloy_als() {
    common::assert_snapshot(&run_on("alloy.als"), &snapshots_dir(), "alloy_als");
}

#[test]
fn lang_apl_apl() {
    common::assert_snapshot(&run_on("apl.apl"), &snapshots_dir(), "apl_apl");
}

#[test]
fn lang_arduino_ino() {
    common::assert_snapshot(&run_on("arduino.ino"), &snapshots_dir(), "arduino_ino");
}

#[test]
fn lang_arturo_art() {
    common::assert_snapshot(&run_on("arturo.art"), &snapshots_dir(), "arturo_art");
}

#[test]
fn lang_asciidoc_adoc() {
    common::assert_snapshot(&run_on("asciidoc.adoc"), &snapshots_dir(), "asciidoc_adoc");
}

#[test]
fn lang_asn1_asn1() {
    common::assert_snapshot(&run_on("asn1.asn1"), &snapshots_dir(), "asn1_asn1");
}

#[test]
fn lang_ats_dats() {
    common::assert_snapshot(&run_on("ats.dats"), &snapshots_dir(), "ats_dats");
}

#[test]
fn lang_awk_awk() {
    common::assert_snapshot(&run_on("awk.awk"), &snapshots_dir(), "awk_awk");
}

#[test]
fn lang_ballerina_bal() {
    common::assert_snapshot(&run_on("ballerina.bal"), &snapshots_dir(), "ballerina_bal");
}

#[test]
fn lang_bazel_bzl() {
    common::assert_snapshot(&run_on("bazel.bzl"), &snapshots_dir(), "bazel_bzl");
}

#[test]
fn lang_bean_bean() {
    common::assert_snapshot(&run_on("bean.bean"), &snapshots_dir(), "bean_bean");
}

#[test]
fn lang_bicep_bicep() {
    common::assert_snapshot(&run_on("bicep.bicep"), &snapshots_dir(), "bicep_bicep");
}

#[test]
fn lang_bitbake_bb() {
    common::assert_snapshot(&run_on("bitbake.bb"), &snapshots_dir(), "bitbake_bb");
}

#[test]
fn lang_bqn_bqn() {
    common::assert_snapshot(&run_on("bqn.bqn"), &snapshots_dir(), "bqn_bqn");
}

#[test]
fn lang_brightscript_brs() {
    assert_eq!(
        run_on("brightscript.brs"),
        load_snapshot("brightscript_brs")
    );
}

#[test]
fn lang_c3_c3() {
    common::assert_snapshot(&run_on("c3.c3"), &snapshots_dir(), "c3_c3");
}

#[test]
fn lang_c_c() {
    common::assert_snapshot(&run_on("c.c"), &snapshots_dir(), "c_c");
}

#[test]
fn lang_cairo_cairo() {
    common::assert_snapshot(&run_on("cairo.cairo"), &snapshots_dir(), "cairo_cairo");
}

#[test]
fn lang_cangjie_cj() {
    common::assert_snapshot(&run_on("cangjie.cj"), &snapshots_dir(), "cangjie_cj");
}

#[test]
fn lang_chapel_chpl() {
    common::assert_snapshot(&run_on("chapel.chpl"), &snapshots_dir(), "chapel_chpl");
}

#[test]
fn lang_cil_cil() {
    common::assert_snapshot(&run_on("cil.cil"), &snapshots_dir(), "cil_cil");
}

#[test]
fn lang_circom_circom() {
    common::assert_snapshot(&run_on("circom.circom"), &snapshots_dir(), "circom_circom");
}

#[test]
fn lang_clojure_clj() {
    common::assert_snapshot(&run_on("clojure.clj"), &snapshots_dir(), "clojure_clj");
}

#[test]
fn lang_clojurec_cljc() {
    common::assert_snapshot(&run_on("clojurec.cljc"), &snapshots_dir(), "clojurec_cljc");
}

#[test]
fn lang_clojurescript_cljs() {
    assert_eq!(
        run_on("clojurescript.cljs"),
        load_snapshot("clojurescript_cljs")
    );
}

#[test]
fn lang_cmake_cmake() {
    common::assert_snapshot(&run_on("cmake.cmake"), &snapshots_dir(), "cmake_cmake");
}

#[test]
fn lang_codeql_ql() {
    common::assert_snapshot(&run_on("codeql.ql"), &snapshots_dir(), "codeql_ql");
}

#[test]
fn lang_cogent_cogent() {
    common::assert_snapshot(&run_on("cogent.cogent"), &snapshots_dir(), "cogent_cogent");
}

#[test]
fn lang_cpp_cpp() {
    common::assert_snapshot(&run_on("cpp.cpp"), &snapshots_dir(), "cpp_cpp");
}

#[test]
fn lang_cppm_cppm() {
    common::assert_snapshot(&run_on("cppm.cppm"), &snapshots_dir(), "cppm_cppm");
}

#[test]
fn lang_crystal_cr() {
    common::assert_snapshot(&run_on("crystal.cr"), &snapshots_dir(), "crystal_cr");
}

#[test]
fn lang_csharp_cs() {
    common::assert_snapshot(&run_on("csharp.cs"), &snapshots_dir(), "csharp_cs");
}

#[test]
fn lang_cuda_cu() {
    common::assert_snapshot(&run_on("cuda.cu"), &snapshots_dir(), "cuda_cu");
}

#[test]
fn lang_cue_cue() {
    common::assert_snapshot(&run_on("cue.cue"), &snapshots_dir(), "cue_cue");
}

#[test]
fn lang_cython_pyx() {
    common::assert_snapshot(&run_on("cython.pyx"), &snapshots_dir(), "cython_pyx");
}

#[test]
fn lang_d2_d2() {
    common::assert_snapshot(&run_on("d2.d2"), &snapshots_dir(), "d2_d2");
}

#[test]
fn lang_d_d() {
    common::assert_snapshot(&run_on("d.d"), &snapshots_dir(), "d_d");
}

#[test]
fn lang_dhall_dhall() {
    common::assert_snapshot(&run_on("dhall.dhall"), &snapshots_dir(), "dhall_dhall");
}

#[test]
fn lang_dreammaker_dm() {
    common::assert_snapshot(&run_on("dreammaker.dm"), &snapshots_dir(), "dreammaker_dm");
}

#[test]
fn lang_dust_dust() {
    common::assert_snapshot(&run_on("dust.dust"), &snapshots_dir(), "dust_dust");
}

#[test]
fn lang_ebuild_ebuild() {
    common::assert_snapshot(&run_on("ebuild.ebuild"), &snapshots_dir(), "ebuild_ebuild");
}

#[test]
fn lang_edgeql_edgeql() {
    common::assert_snapshot(&run_on("edgeql.edgeql"), &snapshots_dir(), "edgeql_edgeql");
}

#[test]
fn lang_edn_edn() {
    common::assert_snapshot(&run_on("edn.edn"), &snapshots_dir(), "edn_edn");
}

#[test]
fn lang_eight_8th() {
    common::assert_snapshot(&run_on("eight.8th"), &snapshots_dir(), "eight_8th");
}

#[test]
fn lang_elvish_elv() {
    common::assert_snapshot(&run_on("elvish.elv"), &snapshots_dir(), "elvish_elv");
}

#[test]
fn lang_emacs_dev_env_ede() {
    assert_eq!(
        run_on("emacs_dev_env.ede"),
        load_snapshot("emacs_dev_env_ede")
    );
}

#[test]
fn lang_emacs_lisp_el() {
    common::assert_snapshot(&run_on("emacs_lisp.el"), &snapshots_dir(), "emacs_lisp_el");
}

#[test]
fn lang_emojicode() {
    common::assert_snapshot(&run_on("emojicode.🍇"), &snapshots_dir(), "emojicode_🍇");
}

#[test]
fn lang_esdl_esdl() {
    common::assert_snapshot(&run_on("esdl.esdl"), &snapshots_dir(), "esdl_esdl");
}

#[test]
fn lang_example_umpl() {
    common::assert_snapshot(&run_on("example.umpl"), &snapshots_dir(), "example_umpl");
}

#[test]
fn lang_factor_factor() {
    common::assert_snapshot(&run_on("factor.factor"), &snapshots_dir(), "factor_factor");
}

#[test]
fn lang_fennel_fnl() {
    common::assert_snapshot(&run_on("fennel.fnl"), &snapshots_dir(), "fennel_fnl");
}

#[test]
fn lang_flatbuffers_fbs() {
    common::assert_snapshot(
        &run_on("flatbuffers.fbs"),
        &snapshots_dir(),
        "flatbuffers_fbs",
    );
}

#[test]
fn lang_forgecfg_cfg() {
    common::assert_snapshot(&run_on("forgecfg.cfg"), &snapshots_dir(), "forgecfg_cfg");
}

#[test]
fn lang_fsharp_fs() {
    common::assert_snapshot(&run_on("fsharp.fs"), &snapshots_dir(), "fsharp_fs");
}

#[test]
fn lang_fstar_fst() {
    common::assert_snapshot(&run_on("fstar.fst"), &snapshots_dir(), "fstar_fst");
}

#[test]
fn lang_ftl_ftl() {
    common::assert_snapshot(&run_on("ftl.ftl"), &snapshots_dir(), "ftl_ftl");
}

#[test]
fn lang_futhark_fut() {
    common::assert_snapshot(&run_on("futhark.fut"), &snapshots_dir(), "futhark_fut");
}

#[test]
fn lang_gdb_gdb() {
    common::assert_snapshot(&run_on("gdb.gdb"), &snapshots_dir(), "gdb_gdb");
}

#[test]
fn lang_gdshader_gdshader() {
    assert_eq!(
        run_on("gdshader.gdshader"),
        load_snapshot("gdshader_gdshader")
    );
}

#[test]
fn lang_gherkin_feature() {
    common::assert_snapshot(
        &run_on("gherkin.feature"),
        &snapshots_dir(),
        "gherkin_feature",
    );
}

#[test]
fn lang_gleam_gleam() {
    common::assert_snapshot(&run_on("gleam.gleam"), &snapshots_dir(), "gleam_gleam");
}

#[test]
fn lang_glimmer_js_gjs() {
    common::assert_snapshot(
        &run_on("glimmer_js.gjs"),
        &snapshots_dir(),
        "glimmer_js_gjs",
    );
}

#[test]
fn lang_glimmer_ts_gts() {
    common::assert_snapshot(
        &run_on("glimmer_ts.gts"),
        &snapshots_dir(),
        "glimmer_ts_gts",
    );
}

#[test]
fn lang_gml_gml() {
    common::assert_snapshot(&run_on("gml.gml"), &snapshots_dir(), "gml_gml");
}

#[test]
fn lang_go_go() {
    common::assert_snapshot(&run_on("go.go"), &snapshots_dir(), "go_go");
}

#[test]
fn lang_gohtml_gohtml() {
    common::assert_snapshot(&run_on("gohtml.gohtml"), &snapshots_dir(), "gohtml_gohtml");
}

#[test]
fn lang_graphql_gql() {
    common::assert_snapshot(&run_on("graphql.gql"), &snapshots_dir(), "graphql_gql");
}

#[test]
fn lang_gwion_gw() {
    common::assert_snapshot(&run_on("gwion.gw"), &snapshots_dir(), "gwion_gw");
}

#[test]
fn lang_haml_haml() {
    common::assert_snapshot(&run_on("haml.haml"), &snapshots_dir(), "haml_haml");
}

#[test]
fn lang_hcl_tf() {
    common::assert_snapshot(&run_on("hcl.tf"), &snapshots_dir(), "hcl_tf");
}

#[test]
fn lang_headache_ha() {
    common::assert_snapshot(&run_on("headache.ha"), &snapshots_dir(), "headache_ha");
}

#[test]
fn lang_hex0_hex0() {
    common::assert_snapshot(&run_on("hex0.hex0"), &snapshots_dir(), "hex0_hex0");
}

#[test]
fn lang_hex1_hex1() {
    common::assert_snapshot(&run_on("hex1.hex1"), &snapshots_dir(), "hex1_hex1");
}

#[test]
fn lang_hex2_hex2() {
    common::assert_snapshot(&run_on("hex2.hex2"), &snapshots_dir(), "hex2_hex2");
}

#[test]
fn lang_hicad_mac() {
    common::assert_snapshot(&run_on("hicad.mac"), &snapshots_dir(), "hicad_mac");
}

#[test]
fn lang_hledger_hledger() {
    common::assert_snapshot(
        &run_on("hledger.hledger"),
        &snapshots_dir(),
        "hledger_hledger",
    );
}

#[test]
fn lang_hpp_hpp() {
    common::assert_snapshot(&run_on("hpp.hpp"), &snapshots_dir(), "hpp_hpp");
}

#[test]
fn lang_html_html() {
    common::assert_snapshot(&run_on("html.html"), &snapshots_dir(), "html_html");
}

#[test]
fn lang_janet_janet() {
    common::assert_snapshot(&run_on("janet.janet"), &snapshots_dir(), "janet_janet");
}

#[test]
fn lang_java_java() {
    common::assert_snapshot(&run_on("java.java"), &snapshots_dir(), "java_java");
}

#[test]
fn lang_javascript_js() {
    common::assert_snapshot(&run_on("javascript.js"), &snapshots_dir(), "javascript_js");
}

#[test]
fn lang_jinja2_j2() {
    common::assert_snapshot(&run_on("jinja2.j2"), &snapshots_dir(), "jinja2_j2");
}

#[test]
fn lang_jq_jq() {
    common::assert_snapshot(&run_on("jq.jq"), &snapshots_dir(), "jq_jq");
}

#[test]
fn lang_jslt_jslt() {
    common::assert_snapshot(&run_on("jslt.jslt"), &snapshots_dir(), "jslt_jslt");
}

#[test]
fn lang_jsonnet_jsonnet() {
    common::assert_snapshot(
        &run_on("jsonnet.jsonnet"),
        &snapshots_dir(),
        "jsonnet_jsonnet",
    );
}

#[test]
fn lang_jupyter_ipynb() {
    common::assert_snapshot(&run_on("jupyter.ipynb"), &snapshots_dir(), "jupyter_ipynb");
}

#[test]
fn lang_justfile() {
    common::assert_snapshot(&run_on("justfile"), &snapshots_dir(), "justfile");
}

#[test]
fn lang_k_k() {
    common::assert_snapshot(&run_on("k.k"), &snapshots_dir(), "k_k");
}

#[test]
fn lang_kaem_kaem() {
    common::assert_snapshot(&run_on("kaem.kaem"), &snapshots_dir(), "kaem_kaem");
}

#[test]
fn lang_kakoune_script_kak() {
    assert_eq!(
        run_on("kakoune_script.kak"),
        load_snapshot("kakoune_script_kak")
    );
}

#[test]
fn lang_koka_kk() {
    common::assert_snapshot(&run_on("koka.kk"), &snapshots_dir(), "koka_kk");
}

#[test]
fn lang_ksh_ksh() {
    common::assert_snapshot(&run_on("ksh.ksh"), &snapshots_dir(), "ksh_ksh");
}

#[test]
fn lang_kvlanguage_kv() {
    common::assert_snapshot(&run_on("kvlanguage.kv"), &snapshots_dir(), "kvlanguage_kv");
}

#[test]
fn lang_lalrpop_lalrpop() {
    common::assert_snapshot(
        &run_on("lalrpop.lalrpop"),
        &snapshots_dir(),
        "lalrpop_lalrpop",
    );
}

#[test]
fn lang_linguafranca_lf() {
    common::assert_snapshot(
        &run_on("linguafranca.lf"),
        &snapshots_dir(),
        "linguafranca_lf",
    );
}

#[test]
fn lang_liquid_liquid() {
    common::assert_snapshot(&run_on("liquid.liquid"), &snapshots_dir(), "liquid_liquid");
}

#[test]
fn lang_livescript_ls() {
    common::assert_snapshot(&run_on("livescript.ls"), &snapshots_dir(), "livescript_ls");
}

#[test]
fn lang_llvm_ll() {
    common::assert_snapshot(&run_on("llvm.ll"), &snapshots_dir(), "llvm_ll");
}

#[test]
fn lang_logtalk_lgt() {
    common::assert_snapshot(&run_on("logtalk.lgt"), &snapshots_dir(), "logtalk_lgt");
}

#[test]
fn lang_lolcode_lol() {
    common::assert_snapshot(&run_on("lolcode.lol"), &snapshots_dir(), "lolcode_lol");
}

#[test]
fn lang_m1_m1() {
    common::assert_snapshot(&run_on("m1.m1"), &snapshots_dir(), "m1_m1");
}

#[test]
fn lang_m4_m4() {
    common::assert_snapshot(&run_on("m4.m4"), &snapshots_dir(), "m4_m4");
}

#[test]
fn lang_menhir_mly() {
    common::assert_snapshot(&run_on("menhir.mly"), &snapshots_dir(), "menhir_mly");
}

#[test]
fn lang_meson_build() {
    common::assert_snapshot(&run_on("meson.build"), &snapshots_dir(), "meson_build");
}

#[test]
fn lang_metal_metal() {
    common::assert_snapshot(&run_on("metal.metal"), &snapshots_dir(), "metal_metal");
}

#[test]
fn lang_mlatu_mlt() {
    common::assert_snapshot(&run_on("mlatu.mlt"), &snapshots_dir(), "mlatu_mlt");
}

#[test]
fn lang_moduledef_def() {
    common::assert_snapshot(&run_on("moduledef.def"), &snapshots_dir(), "moduledef_def");
}

#[test]
fn lang_mojo_mojo() {
    common::assert_snapshot(&run_on("mojo.mojo"), &snapshots_dir(), "mojo_mojo");
}

#[test]
fn lang_monkeyc_mc() {
    common::assert_snapshot(&run_on("monkeyc.mc"), &snapshots_dir(), "monkeyc_mc");
}

#[test]
fn lang_nextflow_nf() {
    common::assert_snapshot(&run_on("nextflow.nf"), &snapshots_dir(), "nextflow_nf");
}

#[test]
fn lang_nqp_nqp() {
    common::assert_snapshot(&run_on("nqp.nqp"), &snapshots_dir(), "nqp_nqp");
}

#[test]
fn lang_odin_odin() {
    common::assert_snapshot(&run_on("odin.odin"), &snapshots_dir(), "odin_odin");
}

#[test]
fn lang_open_policy_agent_rego() {
    assert_eq!(
        run_on("open_policy_agent.rego"),
        load_snapshot("open_policy_agent_rego")
    );
}

#[test]
fn lang_openscad_scad() {
    common::assert_snapshot(&run_on("openscad.scad"), &snapshots_dir(), "openscad_scad");
}

#[test]
fn lang_opentype_fea() {
    common::assert_snapshot(&run_on("opentype.fea"), &snapshots_dir(), "opentype_fea");
}

#[test]
fn lang_org_mode_org() {
    common::assert_snapshot(&run_on("org_mode.org"), &snapshots_dir(), "org_mode_org");
}

#[test]
fn lang_pan_pan() {
    common::assert_snapshot(&run_on("pan.pan"), &snapshots_dir(), "pan_pan");
}

#[test]
fn lang_pcss_pcss() {
    common::assert_snapshot(&run_on("pcss.pcss"), &snapshots_dir(), "pcss_pcss");
}

#[test]
fn lang_pest_pest() {
    common::assert_snapshot(&run_on("pest.pest"), &snapshots_dir(), "pest_pest");
}

#[test]
fn lang_phix_e() {
    common::assert_snapshot(&run_on("phix.e"), &snapshots_dir(), "phix_e");
}

#[test]
fn lang_plantuml_puml() {
    common::assert_snapshot(&run_on("plantuml.puml"), &snapshots_dir(), "plantuml_puml");
}

#[test]
fn lang_pofile_po() {
    common::assert_snapshot(&run_on("pofile.po"), &snapshots_dir(), "pofile_po");
}

#[test]
fn lang_pofile_pot_pot() {
    common::assert_snapshot(
        &run_on("pofile_pot.pot"),
        &snapshots_dir(),
        "pofile_pot_pot",
    );
}

#[test]
fn lang_poke_pk() {
    common::assert_snapshot(&run_on("poke.pk"), &snapshots_dir(), "poke_pk");
}

#[test]
fn lang_pony_pony() {
    common::assert_snapshot(&run_on("pony.pony"), &snapshots_dir(), "pony_pony");
}

#[test]
fn lang_postcss_sss() {
    common::assert_snapshot(&run_on("postcss.sss"), &snapshots_dir(), "postcss_sss");
}

#[test]
fn lang_powershell_ps1() {
    common::assert_snapshot(
        &run_on("powershell.ps1"),
        &snapshots_dir(),
        "powershell_ps1",
    );
}

#[test]
fn lang_pug_pug() {
    common::assert_snapshot(&run_on("pug.pug"), &snapshots_dir(), "pug_pug");
}

#[test]
fn lang_puppet_pp() {
    common::assert_snapshot(&run_on("puppet.pp"), &snapshots_dir(), "puppet_pp");
}

#[test]
fn lang_pyret_arr() {
    common::assert_snapshot(&run_on("pyret.arr"), &snapshots_dir(), "pyret_arr");
}

#[test]
fn lang_python_py() {
    common::assert_snapshot(&run_on("python.py"), &snapshots_dir(), "python_py");
}

#[test]
fn lang_q_q() {
    common::assert_snapshot(&run_on("q.q"), &snapshots_dir(), "q_q");
}

#[test]
fn lang_qml_qml() {
    common::assert_snapshot(&run_on("qml.qml"), &snapshots_dir(), "qml_qml");
}

#[test]
fn lang_racket_rkt() {
    common::assert_snapshot(&run_on("racket.rkt"), &snapshots_dir(), "racket_rkt");
}

#[test]
fn lang_raku_raku() {
    common::assert_snapshot(&run_on("raku.raku"), &snapshots_dir(), "raku_raku");
}

#[test]
fn lang_razor_cshtml() {
    common::assert_snapshot(&run_on("razor.cshtml"), &snapshots_dir(), "razor_cshtml");
}

#[test]
fn lang_razorcomponent_razor() {
    assert_eq!(
        run_on("razorcomponent.razor"),
        load_snapshot("razorcomponent_razor")
    );
}

#[test]
fn lang_redscript_reds() {
    common::assert_snapshot(
        &run_on("redscript.reds"),
        &snapshots_dir(),
        "redscript_reds",
    );
}

#[test]
fn lang_renpy_rpy() {
    common::assert_snapshot(&run_on("renpy.rpy"), &snapshots_dir(), "renpy_rpy");
}

#[test]
fn lang_roc_roc() {
    common::assert_snapshot(&run_on("roc.roc"), &snapshots_dir(), "roc_roc");
}

#[test]
fn lang_ron_ron() {
    common::assert_snapshot(&run_on("ron.ron"), &snapshots_dir(), "ron_ron");
}

#[test]
fn lang_rpmspec_spec() {
    common::assert_snapshot(&run_on("rpmspec.spec"), &snapshots_dir(), "rpmspec_spec");
}

#[test]
fn lang_ruby_html_erb() {
    common::assert_snapshot(&run_on("ruby_html.erb"), &snapshots_dir(), "ruby_html_erb");
}

#[test]
fn lang_ruby_rb() {
    common::assert_snapshot(&run_on("ruby.rb"), &snapshots_dir(), "ruby_rb");
}

#[test]
fn lang_rust_rs() {
    common::assert_snapshot(&run_on("rust.rs"), &snapshots_dir(), "rust_rs");
}

#[test]
fn lang_scheme_scm() {
    common::assert_snapshot(&run_on("scheme.scm"), &snapshots_dir(), "scheme_scm");
}

#[test]
fn lang_shaderlab_shader() {
    assert_eq!(
        run_on("shaderlab.shader"),
        load_snapshot("shaderlab_shader")
    );
}

#[test]
fn lang_shell_sh() {
    common::assert_snapshot(&run_on("shell.sh"), &snapshots_dir(), "shell_sh");
}

#[test]
fn lang_slang_slang() {
    common::assert_snapshot(&run_on("slang.slang"), &snapshots_dir(), "slang_slang");
}

#[test]
fn lang_slint_slint() {
    common::assert_snapshot(&run_on("slint.slint"), &snapshots_dir(), "slint_slint");
}

#[test]
fn lang_solidity_sol() {
    common::assert_snapshot(&run_on("solidity.sol"), &snapshots_dir(), "solidity_sol");
}

#[test]
fn lang_sql_sql() {
    common::assert_snapshot(&run_on("sql.sql"), &snapshots_dir(), "sql_sql");
}

#[test]
fn lang_srecode_srt() {
    common::assert_snapshot(&run_on("srecode.srt"), &snapshots_dir(), "srecode_srt");
}

#[test]
fn lang_stan_stan() {
    common::assert_snapshot(&run_on("stan.stan"), &snapshots_dir(), "stan_stan");
}

#[test]
fn lang_stata_do() {
    common::assert_snapshot(&run_on("stata.do"), &snapshots_dir(), "stata_do");
}

#[test]
fn lang_stratego_str() {
    common::assert_snapshot(&run_on("stratego.str"), &snapshots_dir(), "stratego_str");
}

#[test]
fn lang_stylus_styl() {
    common::assert_snapshot(&run_on("stylus.styl"), &snapshots_dir(), "stylus_styl");
}

#[test]
fn lang_svelte_svelte() {
    common::assert_snapshot(&run_on("svelte.svelte"), &snapshots_dir(), "svelte_svelte");
}

#[test]
fn lang_swift_swift() {
    common::assert_snapshot(&run_on("swift.swift"), &snapshots_dir(), "swift_swift");
}

#[test]
fn lang_swig_i() {
    common::assert_snapshot(&run_on("swig.i"), &snapshots_dir(), "swig_i");
}

#[test]
fn lang_tact_tact() {
    common::assert_snapshot(&run_on("tact.tact"), &snapshots_dir(), "tact_tact");
}

#[test]
fn lang_templ_templ() {
    common::assert_snapshot(&run_on("templ.templ"), &snapshots_dir(), "templ_templ");
}

#[test]
fn lang_thrift_thrift() {
    common::assert_snapshot(&run_on("thrift.thrift"), &snapshots_dir(), "thrift_thrift");
}

#[test]
fn lang_tsx_tsx() {
    common::assert_snapshot(&run_on("tsx.tsx"), &snapshots_dir(), "tsx_tsx");
}

#[test]
fn lang_ttcn_ttcn3() {
    common::assert_snapshot(&run_on("ttcn.ttcn3"), &snapshots_dir(), "ttcn_ttcn3");
}

#[test]
fn lang_twig_twig() {
    common::assert_snapshot(&run_on("twig.twig"), &snapshots_dir(), "twig_twig");
}

#[test]
fn lang_typescript_ts() {
    common::assert_snapshot(&run_on("typescript.ts"), &snapshots_dir(), "typescript_ts");
}

#[test]
fn lang_typst_typ() {
    common::assert_snapshot(&run_on("typst.typ"), &snapshots_dir(), "typst_typ");
}

#[test]
fn lang_uiua_ua() {
    common::assert_snapshot(&run_on("uiua.ua"), &snapshots_dir(), "uiua_ua");
}

#[test]
fn lang_unison_u() {
    common::assert_snapshot(&run_on("unison.u"), &snapshots_dir(), "unison_u");
}

#[test]
fn lang_urweb_ur() {
    common::assert_snapshot(&run_on("urweb.ur"), &snapshots_dir(), "urweb_ur");
}

#[test]
fn lang_urweb_urp_urp() {
    common::assert_snapshot(&run_on("urweb_urp.urp"), &snapshots_dir(), "urweb_urp_urp");
}

#[test]
fn lang_urweb_urs_urs() {
    common::assert_snapshot(&run_on("urweb_urs.urs"), &snapshots_dir(), "urweb_urs_urs");
}

#[test]
fn lang_vb6_bas_bas() {
    common::assert_snapshot(&run_on("vb6_bas.bas"), &snapshots_dir(), "vb6_bas_bas");
}

#[test]
fn lang_vb6_cls_cls() {
    common::assert_snapshot(&run_on("vb6_cls.cls"), &snapshots_dir(), "vb6_cls_cls");
}

#[test]
fn lang_vb6_frm_frm() {
    common::assert_snapshot(&run_on("vb6_frm.frm"), &snapshots_dir(), "vb6_frm_frm");
}

#[test]
fn lang_vbscript_vbs() {
    common::assert_snapshot(&run_on("vbscript.vbs"), &snapshots_dir(), "vbscript_vbs");
}

#[test]
fn lang_velocity_vm() {
    common::assert_snapshot(&run_on("velocity.vm"), &snapshots_dir(), "velocity_vm");
}

#[test]
fn lang_vhdl_vhd() {
    common::assert_snapshot(&run_on("vhdl.vhd"), &snapshots_dir(), "vhdl_vhd");
}

#[test]
fn lang_visualbasic_vb() {
    common::assert_snapshot(
        &run_on("visualbasic.vb"),
        &snapshots_dir(),
        "visualbasic_vb",
    );
}

#[test]
fn lang_vqe_qasm() {
    common::assert_snapshot(&run_on("vqe.qasm"), &snapshots_dir(), "vqe_qasm");
}

#[test]
fn lang_vue_vue() {
    common::assert_snapshot(&run_on("vue.vue"), &snapshots_dir(), "vue_vue");
}

#[test]
fn lang_webassembly_wat() {
    common::assert_snapshot(
        &run_on("webassembly.wat"),
        &snapshots_dir(),
        "webassembly_wat",
    );
}

#[test]
fn lang_wenyan_wy() {
    common::assert_snapshot(&run_on("wenyan.wy"), &snapshots_dir(), "wenyan_wy");
}

#[test]
fn lang_wgsl_wgsl() {
    common::assert_snapshot(&run_on("wgsl.wgsl"), &snapshots_dir(), "wgsl_wgsl");
}

#[test]
fn lang_xsl_xsl() {
    common::assert_snapshot(&run_on("xsl.xsl"), &snapshots_dir(), "xsl_xsl");
}

#[test]
fn lang_xtend_xtend() {
    common::assert_snapshot(&run_on("xtend.xtend"), &snapshots_dir(), "xtend_xtend");
}

#[test]
fn lang_yaml_yaml() {
    common::assert_snapshot(&run_on("yaml.yaml"), &snapshots_dir(), "yaml_yaml");
}

#[test]
fn lang_zencode_zs() {
    common::assert_snapshot(&run_on("zencode.zs"), &snapshots_dir(), "zencode_zs");
}

#[test]
fn lang_zig_zig() {
    common::assert_snapshot(&run_on("zig.zig"), &snapshots_dir(), "zig_zig");
}

#[test]
fn lang_zokrates_zok() {
    common::assert_snapshot(&run_on("zokrates.zok"), &snapshots_dir(), "zokrates_zok");
}
