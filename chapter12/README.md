# 12장 - 병렬 처리(썸네일 생성 도구)

이 폴더는 러스트 인사이트북 **12장 - 병렬 처리(썸네일 생성 도구)**의 예제 코드를 포함하고 있습니다.

## 포함된 프로젝트 및 예제

* **[thumbnail-tool(sync)](./thumbnail-tool(sync))**: 동기 방식의 썸네일 생성 도구
* **[thumbnail-tool(thread)](./thumbnail-tool(thread))**: 멀티스레드 기반 썸네일 생성 도구
* **[thumbnail-tool(channel)](./thumbnail-tool(channel))**: 메시지 패싱(채널) 기반 썸네일 생성 도구
* **[thumbnail-tool(rayon)](./thumbnail-tool(rayon))**: Rayon 크레이트 기반 병렬 처리 썸네일 생성 도구
* **[parallel-add](./parallel-add)**: 병렬 덧셈 예제

## 실행 방법

각 프로젝트 폴더로 이동하여 실행할 수 있습니다. 예를 들어:

```bash
cd thumbnail-tool(sync)
cargo run
```
