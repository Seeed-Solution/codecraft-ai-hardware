# Display refresh implementations

Source: Seeed's [refresh and low-power guide](https://www.seeedstudio.com/sticky/docs/en/device-guide/esp-refresh/) and [ESP-IDF pages guide](https://www.seeedstudio.com/sticky/docs/en/device-guide/esp-pages/). The official APIs below belong to Seeed's dashboard demo; the current repository BSP has a different, smaller API.

## Choose a refresh method

| Content and goal | Official demo API | Behavior and correct use |
| --- | --- | --- |
| Four-level grayscale image or icon | `sticky_display_refresh()` | Full refresh; slow, cleans previous content and may visibly flicker. Required for grayscale. |
| Monochrome page with substantial changes | `sticky_display_refresh_monochrome()` | Full 1-bit refresh; use for page changes or periodic ghosting cleanup. |
| Small monochrome update | `sticky_display_refresh_partial()` | Faster 1-bit partial refresh; keep the unchanged framebuffer content intact. |

For RTC minute updates, the Page layer redraws only the time area while the Display layer submits a complete monochrome comparison frame. See [Partially refresh the RTC status-bar time](display-rtc-time.md). After repeated partial updates, use the page's assigned full refresh when appropriate; in the official demo, a short press of AI / OK / Power button performs that full-page redraw.

## Official demo sequence

1. Draw into the **800 × 480 landscape** Canvas; the official Display layer handles the 180° rotation.
2. Keep refresh selection in the App layer. `refresh_current_page()` chooses four-level grayscale full refresh for Home and monochrome full refresh for the other full pages.
3. Use `sticky_display_refresh_partial()` only for small monochrome updates such as the RTC time, never for grayscale content or complete page changes.
4. Before deep sleep, finish the monochrome sleep-page refresh, stop touch polling, and put the display to sleep. See [Deep sleep and wake](deep-sleep-wake.md).

## Current repository BSP

The current Target supports **SSD1677 only**, with a logical **480x800 portrait** framebuffer.

| Required update | Current API | Important contract |
| --- | --- | --- |
| Monochrome full refresh | `bsp_epd_update(frame, prev, true)` | Slow, clean update. Pass `prev=NULL` after boot or a failed update to force full recovery. |
| Monochrome fast refresh | `bsp_epd_update(frame, prev, false)` | Uses the previous framebuffer; this is not an arbitrary-region partial-refresh API. |
| Four-level grayscale full refresh | `bsp_epd_update_gray4(frame)` | Always a full refresh; the next monochrome update must be full. |

Mono buffers are 1 bit per pixel, MSB first (`1=white`, `0=black`). Gray4 buffers are 2 bits per pixel, MSB first (`0=black` through `3=white`). The caller owns and retains all buffers until the synchronous call returns. Call from the display-owning task, never from an ISR or a non-blocking UI callback; concurrent calls are rejected.

See [`bsp_epd.h`](../targets/bsp/include/bsp_epd.h) for the binding implementation contract. No separate region-based partial-update API is currently exposed.
