use std::sync::mpsc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        // main から各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();
        // 各スレッドからmainへのチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();
        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_tx.clone());

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    //各スレッドにdataの値の送信
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
