PORTNAME=	LWP-Authen-OAuth2
PORTVERSION=	0.20
CATEGORIES=	www perl5
MASTER_SITES=	CPAN
PKGNAMEPREFIX=	p5-

MAINTAINER=	zi@FreeBSD.org
COMMENT=	Perl extension to generate signed OAuth2 requests
WWW=		https://metacpan.org/release/LWP-Authen-OAuth2

LICENSE=	ART20

BUILD_DEPENDS=	${RUN_DEPENDS}
RUN_DEPENDS=	p5-libwww>=0:www/p5-libwww \
		p5-LWP-Protocol-https>0:www/p5-LWP-Protocol-https \
		p5-Mozilla-CA>0:www/p5-Mozilla-CA \
		p5-JSON>0:converters/p5-JSON

USES=		perl5
USE_PERL5=	modbuild

.include <bsd.port.mk>
