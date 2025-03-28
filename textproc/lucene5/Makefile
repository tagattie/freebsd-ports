PORTNAME=	lucene
PORTVERSION=	5.5.5
CATEGORIES=	textproc java devel
MASTER_SITES=	https://archive.apache.org/dist/${PORTNAME}/java/${PORTVERSION}
PKGNAMESUFFIX=	5

MAINTAINER=	ports@FreeBSD.org
COMMENT=	Full-text search engine for Java, 5.x branch
WWW=		https://lucene.apache.org/core/

LICENSE=	APACHE20
LICENSE_FILE=	${WRKSRC}/LICENSE.txt

CONFLICTS=	lucene-[0-9].*

USES=		cpe java tar:tgz

NO_BUILD=	yes
NO_ARCH=	yes

CPE_VENDOR=	apache

PLIST_FILES=	%%JAVAJARDIR%%/${PORTNAME}-core-${PORTVERSION}.jar
PORTDOCS=	*
PORTEXAMPLES=	*

CONTRIBS=	backward-codecs \
		benchmark \
		classification \
		codecs \
		demo \
		expressions \
		facet	\
		grouping \
		highlighter \
		join \
		memory \
		misc \
		queries \
		queryparser \
		replicator \
		sandbox \
		spatial \
		spatial3d \
		suggest

EXTRA_CONTRIBS_DIRS= \
		common \
		icu \
		kuromoji \
		morfologik \
		phonetic \
		smartcn \
		stempel \
		uima

OPTIONS_DEFINE=	CONTRIB DOCS EXAMPLES
OPTIONS_DEFAULT=CONTRIB
CONTRIB_DESC=	Install contributed jars

.for f in ${CONTRIBS}
CONTRIB_PLIST_FILES+=	%%JAVAJARDIR%%/${PORTNAME}-${f}-${PORTVERSION}.jar
.endfor
.for g in ${EXTRA_CONTRIBS_DIRS}
CONTRIB_PLIST_FILES+=	%%JAVAJARDIR%%/${PORTNAME}-analyzers-${g}-${PORTVERSION}.jar
.endfor

do-install:
	${INSTALL_DATA} ${WRKSRC}/core/${PORTNAME}-core-${PORTVERSION}.jar ${STAGEDIR}${JAVAJARDIR}

do-install-CONTRIB-on:
.for f in ${CONTRIBS}
	${INSTALL_DATA} ${WRKSRC}/${f}/${PORTNAME}-${f}-${PORTVERSION}.jar ${STAGEDIR}${JAVAJARDIR}
.endfor
.for d in ${EXTRA_CONTRIBS_DIRS}
	${INSTALL_DATA} ${WRKSRC}/analysis/${d}/${PORTNAME}-analyzers-${d}-${PORTVERSION}.jar ${STAGEDIR}${JAVAJARDIR}
.endfor

do-install-DOCS-on:
	@${MKDIR} ${STAGEDIR}${DOCSDIR}
.for i in CHANGES.txt NOTICE.txt README.txt
	${INSTALL_DATA} ${WRKSRC}/${i} ${STAGEDIR}${DOCSDIR}
.endfor
	(cd ${WRKSRC}/docs && ${COPYTREE_SHARE} \* ${STAGEDIR}${DOCSDIR})

do-install-EXAMPLES-on:
	@${MKDIR} ${STAGEDIR}${EXAMPLESDIR}
	${INSTALL_DATA} ${WRKSRC}/demo/${PORTNAME}-demo-${PORTVERSION}.jar ${STAGEDIR}${EXAMPLESDIR}

.include <bsd.port.mk>
