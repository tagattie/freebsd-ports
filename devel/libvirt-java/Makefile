PORTNAME=	libvirt-java
PORTVERSION=	0.5.3
CATEGORIES=	devel java
MASTER_SITES=	http://libvirt.org/sources/java/ \
		ftp://libvirt.org/libvirt/java/

MAINTAINER=	novel@FreeBSD.org
COMMENT=	Java bindings for Libvirt virtualization API
WWW=		https://libvirt.org/java.html

LICENSE=	MIT

LIB_DEPENDS=	libvirt.so:devel/libvirt
BUILD_DEPENDS=	jna>=4.0:devel/jna \
		junit>=4.8.2:java/junit

USES=		java:ant

ALL_TARGET=	jar

PLIST_FILES=	%%JAVAJARDIR%%/libvirt-${PORTVERSION}.jar
PORTDOCS=	*

OPTIONS_DEFINE=	DOCS

.include <bsd.port.options.mk>

post-patch:
	@${REINPLACE_CMD} -e 's|/usr/share/java|${JAVAJARDIR}|' \
		${WRKSRC}/build.properties

post-build-script:
.if ${PORT_OPTIONS:MDOCS}
	cd ${WRKSRC} && \
		${ANT} docs
.endif

do-install:
	${INSTALL_DATA} ${WRKSRC}/target/libvirt-${PORTVERSION}.jar \
		${STAGEDIR}${JAVAJARDIR}
.if ${PORT_OPTIONS:MDOCS}
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
	cd ${WRKSRC}/target/javadoc && \
		${COPYTREE_SHARE} \* ${STAGEDIR}${DOCSDIR}
.endif

.include <bsd.port.mk>
