PORTNAME=	trove4j
PORTVERSION=	3.1a1
CATEGORIES=	java devel
MASTER_SITES=	https://bitbucket.org/${PORTNAME}/trove/downloads/
DISTNAME=	trove-${PORTVERSION}

MAINTAINER=	ports@FreeBSD.org
COMMENT=	High performance collections API for Java
WWW=		https://bitbucket.org/robeden/trove/

LICENSE=	LGPL21
LICENSE_FILE=	${WRKSRC}/LICENSE.txt

USES=		java

NO_BUILD=	yes

PORTDOCS=	javadocs AUTHORS.txt CHANGES.txt LICENSE.txt README-idea-devel.txt \
		README-license.txt README.txt
JARFILE=	trove-${PORTVERSION}.jar
PLIST_FILES+=	%%JAVAJARDIR%%/${JARFILE}
WRKSRC=		${WRKDIR}/${PORTVERSION}

OPTIONS_DEFINE=	DOCS

do-install:
	${INSTALL_DATA} ${WRKSRC}/lib/${JARFILE} ${STAGEDIR}${JAVAJARDIR}
	cd ${WRKSRC} && ${COPYTREE_SHARE} "${PORTDOCS}" ${STAGEDIR}${DOCSDIR}

.include <bsd.port.mk>
