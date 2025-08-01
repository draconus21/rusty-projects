#[cfg(test)]
mod tests {
    use super::super::RatePlan;

    #[test]
    fn test_create_rate_plan() {
        let r = RatePlan::new("test".into());
        assert_eq!(r.name(), "test");
    }
}
