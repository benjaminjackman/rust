import _str.sbuf;
import _vec.vbuf;

native mod libc = "libc.so.6" {

    fn open(sbuf s, int flags, uint mode) -> int;
    fn read(int fd, vbuf buf, uint count) -> int;
    fn write(int fd, vbuf buf, uint count) -> int;
    fn close(int fd) -> int;

    type FILE;
    fn fopen(sbuf path, sbuf mode) -> FILE;
    fn fclose(FILE f);
    fn fgetc(FILE f) -> int;
    fn ungetc(int c, FILE f);

    type dir;
    // readdir is a mess; handle via wrapper function in rustrt.
    fn opendir(sbuf d) -> dir;
    fn closedir(dir d) -> int;

    fn getenv(sbuf n) -> sbuf;
    fn setenv(sbuf n, sbuf v, int overwrite) -> int;
    fn unsetenv(sbuf n) -> int;
}

mod libc_constants {
    fn O_RDONLY() -> int { ret 0x0000; }
    fn O_WRONLY() -> int { ret 0x0001; }
    fn O_RDWR()   -> int { ret 0x0002; }
    fn O_APPEND() -> int { ret 0x0400; }
    fn O_CREAT()  -> int { ret 0x0040; }
    fn O_EXCL()   -> int { ret 0x0080; }
    fn O_TRUNC()  -> int { ret 0x0200; }
    fn O_TEXT()   -> int { ret 0x0000; } // nonexistent in linux libc
    fn O_BINARY() -> int { ret 0x0000; } // nonexistent in linux libc

    fn S_IRUSR() -> uint { ret 0x0100u; }
    fn S_IWUSR() -> uint { ret 0x0080u; }
}

fn path_sep() -> str {
    ret "/";
}

fn exec_suffix() -> str {
    ret "";
}

fn target_os() -> str {
    ret "linux";
}

// Local Variables:
// mode: rust;
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// compile-command: "make -k -C .. 2>&1 | sed -e 's/\\/x\\//x:\\//g'";
// End:
