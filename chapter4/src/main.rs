// rand::seq::SliceRandom 트레이트를 가져옵니다. 벡터나 슬라이스를 무작위로 섞는(.shuffle()) 기능 등을 제공합니다.
use rand::seq::SliceRandom;

// Copy, Clone, PartialEq, Debug 트레이트를 Suit 열거형에 자동으로 구현(derive)합니다.
// - Copy, Clone: 값 복사 가능
// - PartialEq: suit끼리 비교 연산(==) 가능
// - Debug: println!("{:?}", suit) 형태로 디버그용 출력 가능
#[derive(Copy, Clone, PartialEq, Debug)]
enum Suit {
    Club,    // 클로버
    Diamond, // 다이아몬드
    Heart,   // 하트
    Spade,   // 스페이드
}

// Card 구조체 역시 값 복사, 비교, 디버그 출력이 가능하도록 derive 매크로를 사용합니다.
#[derive(Copy, Clone, PartialEq, Debug)]
struct Card {
    suit: Suit, // 카드의 무늬 (Club, Diamond, Heart, Spade)
    rank: i32,  // 카드의 숫자 (1 ~ 13)
}

fn main() {
    // 카드를 저장할 덱(deck)을 가변(mut) 벡터로 생성합니다.
    let mut deck: Vec<Card> = Vec::new();
    // 포커 카드의 4가지 무늬를 배열로 정의합니다.
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // 덱 작성: 4가지 무늬와 1부터 13까지의 숫자를 조합하여 총 52장의 카드를 생성합니다.
    for suit in suits {
        for rank in 1..=13 {
            // deck 벡터의 끝에 생성한 Card 구조체 인스턴스를 추가합니다.
            deck.push(Card { suit, rank });
        }
    }

    // 덱 섞기: 난수 생성기(rng)를 생성하고 deck 슬라이스를 무작위로 섞습니다.
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);

    // 덱 표시 (주석 처리된 코드)
    // for card in &deck {
    //     println!("{:?} {:}", card.suit, card.rank);
    // }

    // 플레이어가 쥘 카드 5장(패, hand)을 저장할 가변 벡터를 생성합니다.
    let mut hand: Vec<Card> = Vec::new();
    // 덱에서 카드 5장을 뽑아 패에 넣습니다.
    for _ in 0..5 {
        // deck.pop()은 옵셔널(Option<Card>)을 리턴하므로 unwrap()으로 값을 꺼냅니다. (덱이 비어있지 않다고 가정)
        hand.push(deck.pop().unwrap());
    }

    // 패 정렬: 카드의 랭크(rank, 숫자)를 기준으로 오름차순 정렬합니다.
    // cmp 메소드를 사용해 a의 rank와 b의 rank를 비교합니다.
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 현재 가진 패 표시
    println!("---Hand---");
    // hand.iter().enumerate()를 통해 인덱스(0부터 시작)와 카드 객체를 함께 가져옵니다.
    for (i, card) in hand.iter().enumerate() {
        // 사용자 화면에는 1번부터 표시하기 위해 i + 1을 사용합니다.
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    // 사용자에게 교환을 유도하는 출력
    println!("교환하고 싶은 카드 번호를 입력하세요(예: 1 2 3)");

    // 패 교환
    // 표준 입력에서 교환할 카드 번호를 입력받습니다. (예: "1 2 3" 입력 시 1, 2, 3번째 카드를 교환)
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // 입력된 공백 기준 문자열들을 분리하고 각각 정수로 파싱하여 usize 형식의 벡터로 수집(collect)합니다.
    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    // 입력받은 번호의 카드를 덱에서 새로 뽑은 카드로 바꿉니다.
    for number in numbers {
        // 입력 번호는 1-based이므로 0-based 인덱스로 맞추기 위해 number - 1을 사용합니다.
        // 덱(deck) 맨 위에서 카드를 꺼내(pop) 해당 위치의 패에 덮어씁니다.
        hand[number - 1] = deck.pop().unwrap();
    }

    // 패를 다시 숫자를 기준으로 오름차순 정렬합니다.
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 교환 후 최종 패 표시
    println!("---Hand---");
    for card in &hand {
        println!("{:?} {:}", card.suit, card.rank);
    }

    // 플러시(Flush) 확인: 5장의 카드가 모두 동일한 무늬(suit)인지 검사합니다.
    // 패의 첫 번째 카드를 기준으로 잡고 (비어있지 않으므로 unwrap)
    let suit = hand.first().unwrap().suit;
    // 패의 모든(all) 카드가 기준 무늬와 일치하는지 검사합니다.
    let flash = hand.iter().all(|c| c.suit == suit);

    // 페어(Pair) 확인을 위한 카운트 계산
    let mut count = 0;
    // 모든 카드 쌍(i, j)을 비교하여 숫자가 일치하는 경우의 수를 셉니다.
    for i in 0..hand.len() - 1 {
        for j in i + 1..hand.len() {
            // 숫자가 서로 같은 카드가 있으면 카운트를 증가시킵니다.
            if hand[i].rank == hand[j].rank {
                count += 1;
            }
        }
    }

    // 족보 판정 및 결과 출력
    if flash {
        // 모든 카드가 같은 무늬이면 플러시
        println!("플러시!");
    } else if count >= 3 {
        // 같은 숫자의 쌍이 3개 이상이면 쓰리 카드 또는 그 이상 (예: Full House, Four of a kind 등도 여기에 걸림)
        println!("쓰리 카드!");
    } else if count == 2 {
        // 같은 숫자의 쌍이 2개이면 투 페어
        println!("투 페어!");
    } else if count == 1 {
        // 같은 숫자의 쌍이 1개이면 원 페어
        println!("원 페어!");
    } else {
        // 아무것도 해당하지 않는 경우
        println!("노 페어...")
    }
}
