PORTNAME=	mail2sms
PORTVERSION=	1.3.5
PORTREVISION=	2
CATEGORIES=	mail
MASTER_SITES=	http://daniel.haxx.se/projects/mail2sms/ \
		http://bitrote.org/distfiles/

MAINTAINER=	jharris@widomaker.com
COMMENT=	Mail to SMS converter
WWW=		https://daniel.haxx.se/projects/mail2sms/

LICENSE=	GPLv2+
LICENSE_FILE=	${WRKSRC}/LEGAL

GNU_CONFIGURE=	yes

PLIST_FILES=	bin/mail2sms share/man/man1/mail2sms.1.gz share/man/man5/mail2sms.5.gz
PORTDOCS=	README REGEX example.conf

OPTIONS_DEFINE=	DOCS

.include <bsd.port.pre.mk>

.if ${OPSYS} == FreeBSD
CFLAGS+=	-Wno-error=int-conversion
.endif

post-patch:
	${CP} ${WRKSRC}/mail2sms.4 ${WRKSRC}/mail2sms.5
	${REINPLACE_CMD} -e 's|%%PREFIX%%|${PREFIX}|g' ${WRKSRC}/mail2sms.5
	${REINPLACE_CMD} -e 's|%%PREFIX%%|${PREFIX}|g' ${WRKSRC}/main.c

do-install:
	${INSTALL_PROGRAM} ${WRKSRC}/mail2sms ${STAGEDIR}${PREFIX}/bin
	${INSTALL_MAN} ${WRKSRC}/mail2sms.1 ${STAGEDIR}${PREFIX}/share/man/man1
	${INSTALL_MAN} ${WRKSRC}/mail2sms.5 ${STAGEDIR}${PREFIX}/share/man/man5
	${MKDIR} ${STAGEDIR}${DOCSDIR}
	${INSTALL_DATA} ${PORTDOCS:S,^,${WRKSRC}/,} ${STAGEDIR}${DOCSDIR}

.include <bsd.port.post.mk>
