PORTNAME=	kde-dev-scripts
DISTVERSION=	${KDE_APPLICATIONS_VERSION}
CATEGORIES=	devel kde kde-applications

MAINTAINER=	kde@FreeBSD.org
COMMENT=	KDE development scripts

USES=		cmake kde:6 perl5 qt:6 shebangfix tar:xz \
		python:run
USE_KDE=	doctools \
		ecm:build
USE_QT=		base

SHEBANG_FILES=	cxxmetric \
		draw_lib_dependencies \
		fixkdeincludes \
		fixuifiles \
		grantlee_strings_extractor.py \
		kde-systemsettings-tree.py \
		kde_generate_export_header \
		kdelnk2desktop.py \
		krazy-licensecheck \
		png2mng.pl \
		svnintegrate \
		reviewboard-am \
		kde_generate_export_header \
		zonetab2pot.py

OPTIONS_DEFINE=	DOCS

.include <bsd.port.mk>
