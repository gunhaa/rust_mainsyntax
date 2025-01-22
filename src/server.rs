
fn main() {


    let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..14];
    // 같은 역할을 한다. rust는 문자를 모두 가져온다
    // String의 heap은 시작지점의 ptr, length, capacity를 가진다
    // &str의 경우 length는 알고, ptr의 위치만 정해주면 되기 때문에 해당 구문이 성립한다.
    // 러스트의 모든 문자열은 UTF-8로 인코딩되어있다.
    // 즉, 한 문자가 1바이트인지 확실하지 않기때문에 해당 방식은 좋은 방식이 아니다.
    // 해당 10.. 은 10바이트 뒤의 모든 문자를 달라는 의미와 같아, 문제가 생길수있다.
    let string_slice = &string[10..];

    // 해당 예제에서는 문제가 된다
    let emoji = String::from("🙈❤💚💋");

    // 4개를 자를려고했지만, 각 이모지는 하나의 이모지가 4바이트를 차지하기때문에 한개만 가져오게된다
    let emoji_slice = &emoji[..4];

    // String을 받아와, 자동으로 전체를 자른 string slice가 된다.
    let string_borrow: &str = &string;
    
    // 해당 문자열은 immutable하다, 컴파일링 할때 지정해서 크기가 알려진 상태이기 때문이다.
    let string_literal = "1234";

    // 해당 방식은 dbg!에 소유권을 부여하는 방식이고,
    // 해당 코드는 소유권을 이전하려고 하기 때문에 컴파일 오류가 발생한다
    // dbg!(string);
    // 참조를 주는 것으로 해결 할 수 있다.
    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
    dbg!(&emoji);
    dbg!(emoji_slice);

                // 만들려는 Server의 인터페이스
                // Server는 구조체이다.
                // Rust의 구조체는 사용자 지정 데이터 타입이다.
                // 이를 이용해 관련 데이터를 한 곳에 모을 수 있다.
                // 객체 지향 언어의 클래스와 비슷한 역할을 한다.
    let server = Server::new("127.0.0.1:8080".to_string());
    // server.run();
}

// Server 구조체 생성
struct Server {
    addr: String, 
}

// 구조체에 기능을 추가하기위한 구현 블럭
// 클래스가 있고 클래스안에 모든 기능이 담겨있는 것과 다름
// 메서드, 연관함수가 들어갈 수 있다.
// 메서드는 다른 언어 클래스의 메서드와 비슷하고
// 연관 함수는 다른 언어의 static 메서드와 비슷한 역할을 한다.(구조체의 인스턴스가 필요없다)
// new는 연관함수이다.
// ::구문을 사용해 연관함수에 엑세스 한다.
impl Server {
    //메서드는 항상 첫번째 파라미터로 self를 가져간다
    //self는 구조체의 인스턴스를 나타낸다
    //java나 C++에서는 this로 불린다
    // 해당 코드에서는 run이 구조체의 소유권을 갖는다.
    // 소유권을 갖지 않게하기위해서는 &self로 파라미터를 가지면 된다.
    fn run(self){
        println!("Listening on {}", self.addr);
    }

    // new라는 연관함수 직접 구현
    // 흔히 어떤 구조체의 메인 생성자의 이름을 new라고 하고, 그 모범사례를 따르는게 보통이다.
    // Server를 Self로도 표현이 가능하다.
    fn new(addr : String) -> Self{
        Self {
            // addr : addr
            // 필드의 이름이 같다면 생략가능
            addr
        }
    }
}