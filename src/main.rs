use the_remote_viewer::AttackDetection;

fn main() {
    let detector = AttackDetection::new(0.9);

    let result = detector.process_event("DDoS", "192.168.1.100");
    println!("Event Result: {:#?}", result);

    let log = detector.get_audit_log();
    println!("Audit Log History: {:#?}", log);
}
