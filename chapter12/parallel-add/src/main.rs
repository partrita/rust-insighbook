// 표준 라이브러리의 동기화(sync) 및 스레드(thread) 관련 도구를 가져옵니다.
use std::{
    // Arc: 여러 스레드 간에 데이터의 소유권을 안전하게 공유할 수 있는 원자적 참조 카운팅(Atomic Reference Counting) 포인터입니다.
    // Mutex: 여러 스레드가 동시에 데이터에 접근하는 것을 막고, 한 번에 하나의 스레드만 데이터를 수정할 수 있도록 보장하는 상호 배제(Mutual Exclusion) 락입니다.
    sync::{Arc, Mutex},
    // OS 스레드를 생성하고 관리하기 위한 스레드 모듈을 가져옵니다.
    thread,
};

fn main() {
    // 0으로 초기화된 Mutex를 생성하고, 이를 Arc로 감싸서 여러 스레드가 공동으로 소유할 수 있는 counter 변수를 만듭니다.
    let counter = Arc::new(Mutex::new(0));

    // 생성할 스레드들의 핸들(Handle)을 보관하여 나중에 모든 스레드가 종료될 때까지 기다릴 수 있도록 벡터를 선언합니다.
    let mut handles = vec![];

    // 4개의 스레드를 생성하기 위한 루프를 돌립니다.
    for _ in 0..4 {
        // Arc의 참조 카운트를 증가시켜 새로운 스레드가 counter에 접근할 수 있도록 포인터를 복제(Clone)합니다.
        // 실제 데이터인 Mutex(0)은 하나만 존재하고 복제되지 않으며, 단지 이를 가리키는 포인터만 복제됩니다.
        let counter = counter.clone();

        // thread::spawn을 사용하여 새 스레드를 생성하고 실행합니다.
        // move 키워드는 클로저(익명 함수) 내부에서 외부 변수(여기서는 복제된 counter)를 사용할 때 소유권을 스레드 안으로 완전히 이전(move)시킵니다.
        handles.push(thread::spawn(move || {
            // 각 스레드는 250,000,000번 반복하는 루프를 실행합니다.
            // 러스트에서는 숫자 가독성을 위해 언더바(_)를 사용할 수 있습니다.
            for _ in 0..2_5000_0000 {
                // Mutex의 lock()을 호출하여 데이터에 접근하기 위한 잠금(락)을 획득합니다.
                // lock()은 성공 시 락을 관리하는 스마트 포인터인 MutexGuard를 반환합니다.
                // 다른 스레드에서 패닉이 발생해 뮤텍스가 손상(poisoned)된 경우를 위해 unwrap()으로 에러를 처리하여 값에 바로 접근합니다.
                let mut writer = counter.lock().unwrap();

                // 역참조 연산자(*)를 사용하여 Mutex 내부의 값에 접근하고 값을 1 증가시킵니다.
                *writer += 1;

                // writer 변수가 스레드의 이 반복문 블록을 벗어나면(Drop), 뮤텍스 가드가 해제되어 다른 스레드가 락을 획득할 수 있게 됩니다.
            }
        }));
    }

    // handles 벡터에 저장된 스레드 핸들을 순회하며 모든 스레드의 종료를 기다립니다.
    for handle in handles {
        // join()은 스레드가 작업을 끝마치고 완전히 종료될 때까지 현재 스레드(메인 스레드)를 대기(블로킹)시킵니다.
        // 스레드가 정상적으로 종료되지 않고 패닉이 발생한 경우, unwrap()을 통해 메인 스레드도 함께 패닉을 발생시켜 프로그램을 종료합니다.
        handle.join().unwrap();
    }

    // 모든 스레드의 실행이 끝난 후 최종 counter 값을 출력합니다.
    // as_ref()로 Arc가 가리키는 내부 Mutex의 참조를 가져온 뒤, lock().unwrap()으로 최종 락을 획득하고 출력합니다.
    println!("counter = {}", counter.as_ref().lock().unwrap());
}
