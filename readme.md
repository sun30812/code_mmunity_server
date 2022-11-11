# Co{de}mmunity Server

코드로 통하는 우리들의 커뮤니티.  
코드뮤니티의 서버입니다.

이곳에서는 클라이언트에서 보낸 요청에 대한 처리 동작
코드가 있습니다.

코드뮤니티 프로젝트를 확인하시려면 [여기](https://github.com/sun30812/code_mmunity)를
확인해보세요!
## 코드뮤니티 소개
코드만으로 다른 사람들과 소통을 시작해보세요!

자신의 소스코드를 뽐내는 것도 좋고, 본인이 작성한 소스코드의 문제점을 물어봐도 좋고, 문제를 내도 좋습니다.
뭐든 좋아요!

코드뮤니티는 댓글과 코드 작성밖에 없습니다. 자신이 원하는게 있으면 주석으로 말하면 되거든요.
주석으로 전달하기 뭔가 어색하다면 본인 포스트에 댓글을 달아도 된답니다.

## 소스코드 설명
해당 프로젝트는 개발의 용이성을 위해 새로 만든 클래스나 메서드에는 전부 주석이 달려있습니다.  
또한 [이 사이트](https://sn30-code-mmunity-server-doc.web.app/code_mmunity_server/index.html)에 방문하시면
위키 형태로 API문서 확인이 가능합니다.

## 컨테이너 제작하기
코드뮤니터 백엔드 서버를 구동하는 컨테이너를 제작할 수 있습니다.  
해당 저장소를 복제하신 후 폴더에 들어가서 `docker build -t code_mmunity_server .` 를 입력하시면 컨테이너를 빌드할 수 있습니다.
### MySql서버 접속에 인증서 파일이 필요한 경우

1. 복제하신 저장소에 cert라는 이름의 폴더를 만들고 만든 폴더에 `DigiCertGlobalRootCA.crt.pem` 파일을 넣어줍니다.
2. `USE_SSL`환경 변수를 `true`로 지정합니다.(모든 환경변수에 대한 설명은 [환경변수](#환경변수)를 참고하세요)

### 환경변수

| 환경변수      | 기본값      | 설명                                                                                |
| ------------- | ----------- | ----------------------------------------------------------------------------------- |
| `APP_PORT`    | `8080`      | 백엔드 통신에 사용할 포트이다. docker에서 **이 포트를 expose시켜야 정상 작동한다.** |
| `DB_DATABASE` | `test`      | MySQL서버의 DB이름이다.                                                             |
| `DB_PASSWD`   | `0000`      | MySQL서버에서 DB에 권한이 부여된 사용자의 비밀번호이다.                             |
| `DB_PORT`     | `3306`      | DB에 접속하기 위한 포트 번호이다.                                                   |
| `DB_SERVER`   | `localhost` | MySQL서버에 접근하기 위한 주소이다.                                                 |
| `DB_USER`     | `test`      | MySQL서버에서 DB에 권한이 부여된 사용자의 ID이다.                                   |
| `USE_SSL`     | `false`     | MySQL서버에 접근할 때 인증서 파일이 필요한지 여부이다. 만일 필요한 경우에는 `true`로 지정하면 된다.                                                                                    |

