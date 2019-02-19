# $FreeBSD$

PORTNAME=	gifski
PORTVERSION=	0.8.7.g20190112
CATEGORIES=	graphics

MAINTAINER=	tagattie@yandex.com
COMMENT=	Highest-quality GIF encoder based on pngquant

LICENSE=	AGPLv3+
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		cargo

USE_GITHUB=	yes
GH_ACCOUNT=	ImageOptim
GH_TAGNAME=	eb97793
GH_TUPLE=	kornelski:rust-ffmpeg:0eea6d01eab7baf35b44573b163524fb323fd9c3:ffmpeg \
		meh:rust-ffmpeg-sys:2937563482b8d3be302ee39c0fbb024e5408b085:ffmpegsys

CARGO_CRATES=	aho-corasick-0.6.9 \
		ansi_term-0.11.0 \
		arrayvec-0.4.10 \
		atty-0.2.11 \
		autocfg-0.1.1 \
		backtrace-0.3.13 \
		backtrace-sys-0.1.28 \
		bindgen-0.32.3 \
		bitflags-0.9.1 \
		bitflags-1.0.4 \
		cc-1.0.28 \
		cexpr-0.2.3 \
		cfg-if-0.1.6 \
		clang-sys-0.21.2 \
		clap-2.32.0 \
		color_quant-1.0.1 \
		crossbeam-deque-0.2.0 \
		crossbeam-epoch-0.3.1 \
		crossbeam-utils-0.2.2 \
		either-1.5.0 \
		env_logger-0.4.3 \
		error-chain-0.12.0 \
		gif-0.10.1 \
		gif-dispose-2.1.1 \
		glob-0.2.11 \
		imagequant-2.11.9 \
		imagequant-sys-2.12.2 \
		imgref-1.3.5 \
		kernel32-sys-0.2.2 \
		lazy_static-1.2.0 \
		libc-0.2.46 \
		libloading-0.4.3 \
		lodepng-2.4.2 \
		log-0.3.9 \
		log-0.4.6 \
		lzw-0.10.0 \
		memchr-1.0.2 \
		memchr-2.1.2 \
		memoffset-0.2.1 \
		natord-1.0.9 \
		nodrop-0.1.13 \
		nom-3.2.1 \
		num_cpus-1.9.0 \
		openmp-sys-0.1.6 \
		pbr-1.0.1 \
		peeking_take_while-0.1.2 \
		pkg-config-0.3.14 \
		proc-macro2-0.2.3 \
		quote-0.4.2 \
		rayon-1.0.3 \
		rayon-core-1.4.1 \
		redox_syscall-0.1.50 \
		redox_termios-0.1.1 \
		regex-0.2.11 \
		regex-syntax-0.5.6 \
		resize-0.3.0 \
		rgb-0.8.11 \
		rustc-demangle-0.1.13 \
		scopeguard-0.3.3 \
		strsim-0.7.0 \
		termion-1.5.1 \
		textwrap-0.10.0 \
		thread_local-0.3.6 \
		time-0.1.42 \
		ucd-util-0.1.3 \
		unicode-width-0.1.5 \
		unicode-xid-0.1.0 \
		utf8-ranges-1.0.2 \
		vec_map-0.8.1 \
		version_check-0.1.5 \
		which-1.0.5 \
		wild-2.0.1 \
		winapi-0.2.8 \
		winapi-0.3.6 \
		winapi-build-0.1.1 \
		winapi-i686-pc-windows-gnu-0.4.0 \
		winapi-x86_64-pc-windows-gnu-0.4.0
CARGO_USE_GITHUB=	yes
CARGO_GH_CARGOTOML=	${CARGO_CARGOTOML} \
			${WRKSRC_ffmpeg}/Cargo.toml \
			${WRKSRC_ffmpegsys}/Cargo.toml

# Can't compile built-in video decoding with FFMpeg
OPTIONS_DEFINE=	DOCS OPENMP # FFMPEG
OPTIONS_DEFAULT=	OPENMP # FFMPEG

# FFMPEG_DESC=	Enable built-in video decoding
# FFMPEG_LIB_DEPENDS=	libavcodec.so:multimedia/ffmpeg
# FFMPEG_VARS=	CARGO_FEATURES+=video

OPENMP_DESC=	Enable OpenMP (parallel processing)
OPENMP_USES=	compiler:openmp
OPENMP_VARS=	CARGO_FEATURES+=openmp

PORTDOCS=	README.md

PLIST_FILES=	bin/gifski

post-install:
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/bin/gifski

post-install-DOCS-on:
	${MKDIR} ${STAGEDIR}${DOCSDIR}
.for doc in ${PORTDOCS}
	${INSTALL_DATA} ${WRKSRC}/${doc} ${STAGEDIR}${DOCSDIR}
.endfor

.include <bsd.port.mk>
