#!/bin/bash
OUT="E:/vue/test/cashier/HarUI/docs/visual-baseline/ep"
mkdir -p "$OUT"
echo "capturing affix..."
agent-browser open "https://element-plus.org/zh-CN/component/affix.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/affix.png" 2>/dev/null || true

echo "capturing alert..."
agent-browser open "https://element-plus.org/zh-CN/component/alert.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/alert.png" 2>/dev/null || true

echo "capturing avatar..."
agent-browser open "https://element-plus.org/zh-CN/component/avatar.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/avatar.png" 2>/dev/null || true

echo "capturing backtop..."
agent-browser open "https://element-plus.org/zh-CN/component/backtop.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/backtop.png" 2>/dev/null || true

echo "capturing badge..."
agent-browser open "https://element-plus.org/zh-CN/component/badge.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/badge.png" 2>/dev/null || true

echo "capturing breadcrumb..."
agent-browser open "https://element-plus.org/zh-CN/component/breadcrumb.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/breadcrumb.png" 2>/dev/null || true

echo "capturing button..."
agent-browser open "https://element-plus.org/zh-CN/component/button.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/button.png" 2>/dev/null || true

echo "capturing calendar..."
agent-browser open "https://element-plus.org/zh-CN/component/calendar.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/calendar.png" 2>/dev/null || true

echo "capturing card..."
agent-browser open "https://element-plus.org/zh-CN/component/card.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/card.png" 2>/dev/null || true

echo "capturing carousel..."
agent-browser open "https://element-plus.org/zh-CN/component/carousel.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/carousel.png" 2>/dev/null || true

echo "capturing cascader..."
agent-browser open "https://element-plus.org/zh-CN/component/cascader.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/cascader.png" 2>/dev/null || true

echo "capturing checkbox..."
agent-browser open "https://element-plus.org/zh-CN/component/checkbox.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/checkbox.png" 2>/dev/null || true

echo "capturing collapse..."
agent-browser open "https://element-plus.org/zh-CN/component/collapse.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/collapse.png" 2>/dev/null || true

echo "capturing color_picker..."
agent-browser open "https://element-plus.org/zh-CN/component/color-picker.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/color_picker.png" 2>/dev/null || true

echo "capturing date_picker..."
agent-browser open "https://element-plus.org/zh-CN/component/date-picker.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/date_picker.png" 2>/dev/null || true

echo "capturing descriptions..."
agent-browser open "https://element-plus.org/zh-CN/component/descriptions.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/descriptions.png" 2>/dev/null || true

echo "capturing divider..."
agent-browser open "https://element-plus.org/zh-CN/component/divider.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/divider.png" 2>/dev/null || true

echo "capturing dropdown..."
agent-browser open "https://element-plus.org/zh-CN/component/dropdown.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/dropdown.png" 2>/dev/null || true

echo "capturing empty..."
agent-browser open "https://element-plus.org/zh-CN/component/empty.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/empty.png" 2>/dev/null || true

echo "capturing input..."
agent-browser open "https://element-plus.org/zh-CN/component/input.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/input.png" 2>/dev/null || true

echo "capturing input_number..."
agent-browser open "https://element-plus.org/zh-CN/component/input-number.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/input_number.png" 2>/dev/null || true

echo "capturing link..."
agent-browser open "https://element-plus.org/zh-CN/component/link.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/link.png" 2>/dev/null || true

echo "capturing page_header..."
agent-browser open "https://element-plus.org/zh-CN/component/page-header.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/page_header.png" 2>/dev/null || true

echo "capturing pagination..."
agent-browser open "https://element-plus.org/zh-CN/component/pagination.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/pagination.png" 2>/dev/null || true

echo "capturing progress..."
agent-browser open "https://element-plus.org/zh-CN/component/progress.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/progress.png" 2>/dev/null || true

echo "capturing rate..."
agent-browser open "https://element-plus.org/zh-CN/component/rate.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/rate.png" 2>/dev/null || true

echo "capturing result..."
agent-browser open "https://element-plus.org/zh-CN/component/result.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/result.png" 2>/dev/null || true

echo "capturing scrollbar..."
agent-browser open "https://element-plus.org/zh-CN/component/scrollbar.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/scrollbar.png" 2>/dev/null || true

echo "capturing select..."
agent-browser open "https://element-plus.org/zh-CN/component/select.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/select.png" 2>/dev/null || true

echo "capturing skeleton..."
agent-browser open "https://element-plus.org/zh-CN/component/skeleton.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/skeleton.png" 2>/dev/null || true

echo "capturing slider..."
agent-browser open "https://element-plus.org/zh-CN/component/slider.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/slider.png" 2>/dev/null || true

echo "capturing space..."
agent-browser open "https://element-plus.org/zh-CN/component/space.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/space.png" 2>/dev/null || true

echo "capturing statistic..."
agent-browser open "https://element-plus.org/zh-CN/component/statistic.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/statistic.png" 2>/dev/null || true

echo "capturing steps..."
agent-browser open "https://element-plus.org/zh-CN/component/steps.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/steps.png" 2>/dev/null || true

echo "capturing switch..."
agent-browser open "https://element-plus.org/zh-CN/component/switch.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/switch.png" 2>/dev/null || true

echo "capturing table..."
agent-browser open "https://element-plus.org/zh-CN/component/table.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/table.png" 2>/dev/null || true

echo "capturing tabs..."
agent-browser open "https://element-plus.org/zh-CN/component/tabs.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/tabs.png" 2>/dev/null || true

echo "capturing tag..."
agent-browser open "https://element-plus.org/zh-CN/component/tag.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/tag.png" 2>/dev/null || true

echo "capturing time_picker..."
agent-browser open "https://element-plus.org/zh-CN/component/time-picker.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/time_picker.png" 2>/dev/null || true

echo "capturing timeline..."
agent-browser open "https://element-plus.org/zh-CN/component/timeline.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/timeline.png" 2>/dev/null || true

echo "capturing tree..."
agent-browser open "https://element-plus.org/zh-CN/component/tree.html" 2>/dev/null || true
agent-browser wait --load networkidle 2>/dev/null || true
agent-browser wait 2000 2>/dev/null || true
agent-browser screenshot "$OUT/tree.png" 2>/dev/null || true

echo "CAPTURE_DONE"