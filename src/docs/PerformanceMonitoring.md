# Performance Monitoring in StoryWeaver

StoryWeaver includes a comprehensive performance monitoring system to help track and optimize application performance. This document explains how to use these features in your development workflow.

## Backend Performance Monitoring

### Measuring Function Execution Time

Use the `PerformanceTimer` to measure the execution time of functions:

```rust
use crate::utils::performance_monitor::PerformanceTimer;
use crate::database::models::ComponentType;

async fn some_function() -> Result<()> {
    // Create a timer for this function
    let timer = PerformanceTimer::new("function_name", ComponentType::System);
    
    // Your function logic here
    // ...
    
    // Stop the timer and record the metric
    timer.stop().await?;
    
    Ok(())
}
```

### Measuring Database Query Performance

Track database query performance using the `measure_query` function:

```rust
use crate::utils::performance_monitor::measure_query;
use crate::database::models::QueryType;

async fn fetch_data() -> Result<Vec<Document>> {
    let query = "SELECT * FROM documents WHERE project_id = ?";
    
    let result = measure_query(
        query,
        QueryType::Select,
        "documents",
        None,
        sqlx::query_as::<_, Document>(query)
            .bind(project_id)
            .fetch_all(pool)
    ).await?;
    
    Ok(result)
}
```

### Detecting Performance Bottlenecks

The system automatically detects performance bottlenecks based on predefined thresholds. You can also manually check for bottlenecks:

```rust
use crate::utils::performance_monitor::BottleneckDetector;
use crate::database::models::ComponentType;

async fn check_performance(value: f64) -> Result<()> {
    let detector = BottleneckDetector::new();
    
    if let Some(bottleneck) = detector.check_metric(
        ComponentType::Database,
        "query_execution",
        value
    ).await? {
        println!("Bottleneck detected: {:?}", bottleneck);
    }
    
    Ok(())
}
```

## Frontend Performance Monitoring

### Monitoring Component Performance

Use the `usePerformanceMonitoring` hook to track component performance:

```tsx
import { usePerformanceMonitoring } from '../hooks/usePerformanceMonitoring';

const MyComponent: React.FC = () => {
    const { trackEvent, trackMetric } = usePerformanceMonitoring('MyComponent', {
        component: 'ui',
        trackRender: true,
        trackMemory: true,
        sampleRate: 0.1 // Only track 10% of renders
    });
    
    // Track a custom event
    const handleClick = () => {
        trackEvent('button_click', true); // Start timing
        
        // Do something
        
        trackEvent('button_click', false); // End timing
    };
    
    // Track a custom metric
    const handleDataLoad = (items: any[]) => {
        trackMetric('items_loaded', items.length);
    };
    
    return (
        <div>
            <button onClick={handleClick}>Click Me</button>
        </div>
    );
};
```

### Performance Settings

Users can configure performance monitoring settings in the application settings:

1. Navigate to Settings > Performance Monitoring
2. Toggle different monitoring features on/off
3. View performance summary
4. Clean up old metrics

## Best Practices

1. **Be Selective**: Don't monitor everything. Focus on critical paths and potential bottlenecks.

2. **Use Sampling**: For high-frequency operations, use sampling to reduce overhead.

3. **Set Appropriate Thresholds**: Adjust bottleneck detection thresholds based on your application's needs.

4. **Regular Maintenance**: Periodically clean up old metrics to prevent database bloat.

5. **Monitor in Production**: Consider keeping lightweight monitoring enabled in production to catch real-world issues.

## Performance Metrics to Watch

- **UI Render Times**: Components taking >100ms to render may need optimization
- **Database Query Times**: Queries taking >100ms are considered slow
- **Memory Usage**: Watch for unexpected increases in memory consumption
- **AI Response Times**: AI operations are expected to be slower, but should still be monitored

## Troubleshooting

If you encounter performance issues:

1. Check the Performance Summary in Settings
2. Look for active bottlenecks
3. Analyze metrics for the affected component
4. Use the browser's performance tools for additional insights
