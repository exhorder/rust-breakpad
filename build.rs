extern crate gcc;

fn main() {
    // Breakpad disASM
    gcc::Build::new()
        .warnings(false)
        .file("breakpad/third_party/libdisasm/ia32_implicit.c")
        .file("breakpad/third_party/libdisasm/ia32_insn.c")
        .file("breakpad/third_party/libdisasm/ia32_invariant.c")
        .file("breakpad/third_party/libdisasm/ia32_modrm.c")
        .file("breakpad/third_party/libdisasm/ia32_opcode_tables.c")
        .file("breakpad/third_party/libdisasm/ia32_operand.c")
        .file("breakpad/third_party/libdisasm/ia32_reg.c")
        .file("breakpad/third_party/libdisasm/ia32_settings.c")
        .file("breakpad/third_party/libdisasm/x86_disasm.c")
        .file("breakpad/third_party/libdisasm/x86_format.c")
        .file("breakpad/third_party/libdisasm/x86_imm.c")
        .file("breakpad/third_party/libdisasm/x86_insn.c")
        .file("breakpad/third_party/libdisasm/x86_misc.c")
        .file("breakpad/third_party/libdisasm/x86_operand_list.c")
        .compile("libdisasm.a");

    // Breakpad Processor Base
    gcc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("-std=c++14")
        .include("breakpad")
        .define("BPLOG_MINIMUM_SEVERITY", Some("SEVERITY_ERROR"))
        .file("breakpad/processor/basic_code_modules.cc")
        .file("breakpad/processor/call_stack.cc")
        .file("breakpad/processor/cfi_frame_info.cc")
        .file("breakpad/processor/disassembler_x86.cc")
        .file("breakpad/processor/dump_context.cc")
        .file("breakpad/processor/dump_object.cc")
        .file("breakpad/processor/fast_source_line_resolver.cc")
        .file("breakpad/processor/logging.cc")
        .file("breakpad/processor/pathname_stripper.cc")
        .file("breakpad/processor/process_state.cc")
        .file("breakpad/processor/proc_maps_linux.cc")
        .file("breakpad/processor/simple_symbol_supplier.cc")
        .file("breakpad/processor/source_line_resolver_base.cc")
        .file("breakpad/processor/stack_frame_cpu.cc")
        .file("breakpad/processor/stack_frame_symbolizer.cc")
        .file("breakpad/processor/stackwalker.cc")
        .file("breakpad/processor/stackwalker_amd64.cc")
        .file("breakpad/processor/stackwalker_arm.cc")
        .file("breakpad/processor/stackwalker_arm64.cc")
        .file("breakpad/processor/stackwalker_mips.cc")
        .file("breakpad/processor/stackwalker_ppc.cc")
        .file("breakpad/processor/stackwalker_ppc64.cc")
        .file("breakpad/processor/stackwalker_sparc.cc")
        .file("breakpad/processor/stackwalker_x86.cc")
        .file("breakpad/processor/tokenize.cc")
        .compile("libprocessor.a");

    // Breakpad Minidump Processor
    gcc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("-std=c++14")
        .include("breakpad")
        .define("BPLOG_MINIMUM_SEVERITY", Some("SEVERITY_ERROR"))
        .file("breakpad/processor/exploitability.cc")
        .file("breakpad/processor/exploitability_linux.cc")
        .file("breakpad/processor/exploitability_win.cc")
        .file("breakpad/processor/minidump.cc")
        .file("breakpad/processor/minidump_processor.cc")
        .file("breakpad/processor/symbolic_constants_win.cc")
        .compile("libminidump.a");

    // Our own library
    gcc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .include("breakpad")
        .file("cpp/bindings.cpp")
        .compile("libbreakpad.a");
}
