# $FreeBSD$

PORTNAME=	gifski
DISTVERSION=	0.9.3
CATEGORIES=	graphics

MAINTAINER=	tagattie@yandex.com
COMMENT=	Highest-quality GIF encoder based on pngquant

LICENSE=	AGPLv3+
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		cargo

USE_GITHUB=	yes
GH_ACCOUNT=	ImageOptim
GH_TUPLE=	kornelski:rust-ffmpeg:0eea6d01eab7baf35b44573b163524fb323fd9c3:ffmpeg \
		meh:rust-ffmpeg-sys:4f14151b9b8134f1f029d49d02cbea5c7337dedb:ffmpegsys

CARGO_CRATES=	aho-corasick-0.6.10 \
		ansi_term-0.11.0 \
		atty-0.2.13 \
		autocfg-0.1.7 \
		backtrace-0.3.40 \
		backtrace-sys-0.1.32 \
		bindgen-0.32.3 \
		bitflags-0.9.1 \
		bitflags-1.2.1 \
		cc-1.0.47 \
		cexpr-0.2.3 \
		cfg-if-0.1.10 \
		clang-sys-0.21.2 \
		clap-2.33.0 \
		color_quant-1.0.1 \
		crossbeam-deque-0.7.2 \
		crossbeam-epoch-0.8.0 \
		crossbeam-queue-0.1.2 \
		crossbeam-utils-0.6.6 \
		crossbeam-utils-0.7.0 \
		either-1.5.3 \
		env_logger-0.4.3 \
		error-chain-0.12.1 \
		gif-0.10.3 \
		gif-dispose-2.2.0 \
		gifsicle-1.92.0 \
		glob-0.2.11 \
		glob-0.3.0 \
		hermit-abi-0.1.3 \
		imagequant-2.11.9 \
		imagequant-sys-2.12.5 \
		imgref-1.4.0 \
		kernel32-sys-0.2.2 \
		lazy_static-1.4.0 \
		libc-0.2.65 \
		libloading-0.4.3 \
		lodepng-2.5.0 \
		log-0.3.9 \
		log-0.4.8 \
		lzw-0.10.0 \
		memchr-1.0.2 \
		memchr-2.2.1 \
		memoffset-0.5.3 \
		natord-1.0.9 \
		nom-3.2.1 \
		num_cpus-1.11.0 \
		numtoa-0.1.0 \
		openmp-sys-0.1.6 \
		pbr-1.0.2 \
		peeking_take_while-0.1.2 \
		pkg-config-0.3.17 \
		proc-macro2-0.2.3 \
		quote-0.4.2 \
		rayon-1.2.0 \
		rayon-core-1.6.0 \
		redox_syscall-0.1.56 \
		redox_termios-0.1.1 \
		regex-0.2.11 \
		regex-syntax-0.5.6 \
		resize-0.3.0 \
		rgb-0.8.14 \
		rustc-demangle-0.1.16 \
		rustc_version-0.2.3 \
		scopeguard-1.0.0 \
		semver-0.9.0 \
		semver-parser-0.7.0 \
		strsim-0.8.0 \
		termion-1.5.3 \
		textwrap-0.11.0 \
		thread_local-0.3.6 \
		time-0.1.42 \
		ucd-util-0.1.5 \
		unicode-width-0.1.6 \
		unicode-xid-0.1.0 \
		utf8-ranges-1.0.4 \
		vec_map-0.8.1 \
		version_check-0.1.5 \
		which-1.0.5 \
		wild-2.0.2 \
		winapi-0.2.8 \
		winapi-0.3.8 \
		winapi-build-0.1.1 \
		winapi-i686-pc-windows-gnu-0.4.0 \
		winapi-x86_64-pc-windows-gnu-0.4.0
CARGO_USE_GITHUB=	yes

# gifski offers built-in video decoding option. But it requires ffmpeg 3.x and
# can't compile with ffmpeg 4.x. So FFMPEG option is currently disabled.
OPTIONS_DEFINE=	DOCS LIBGIFSKI LOSSY OPENMP # FFMPEG
OPTIONS_DEFAULT=	LOSSY OPENMP # FFMPEG

# FFMPEG_DESC=	Enable built-in video decoding
# FFMPEG_LIB_DEPENDS=	libavcodec.so:multimedia/ffmpeg
# FFMPEG_VARS=	CARGO_FEATURES+=video

LIBGIFSKI_DESC=	Install gifski header and library
LIBGIFSKI_PLIST_FILES=	include/gifski.h lib/libgifski.a lib/libgifski.so

LOSSY_DESC=	Enable lossy compression support
LOSSY_VARS=	CARGO_FEATURES+=gifsicle

OPENMP_DESC=	Enable OpenMP (parallel processing)
OPENMP_USES=	compiler:openmp
OPENMP_VARS=	CARGO_FEATURES+=openmp

PORTDOCS=	README.md

PLIST_FILES=	bin/gifski

post-install:
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/gifski

post-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${PORTDOCS:S|^|${WRKSRC}/|} ${STAGEDIR}${DOCSDIR}

post-install-LIBGIFSKI-on:
	${INSTALL_DATA} ${WRKSRC}/gifski.h ${STAGEDIR}${PREFIX}/include
	${INSTALL_DATA} ${WRKDIR}/target/release/libgifski.a ${STAGEDIR}${PREFIX}/lib
	${INSTALL_LIB} ${WRKDIR}/target/release/libgifski.so ${STAGEDIR}${PREFIX}/lib

.include <bsd.port.mk>
