/**
 * 标签颜色生成工具
 * 为不同标签生成一致且美观的颜色
 */

// 预设的柔和色彩调色板
const COLOR_PALETTE = [
  { bg: '#fff2e8', text: '#fa8c16', border: '#ffd591' }, // 橙色
  { bg: '#e6f7ff', text: '#1890ff', border: '#91d5ff' }, // 蓝色
  { bg: '#f6ffed', text: '#52c41a', border: '#b7eb8f' }, // 绿色
  { bg: '#fff2f0', text: '#ff4d4f', border: '#ffccc7' }, // 红色
  { bg: '#f9f0ff', text: '#722ed1', border: '#d3adf7' }, // 紫色
  { bg: '#fff0f6', text: '#eb2f96', border: '#ffadd2' }, // 粉色
  { bg: '#e6fffb', text: '#13c2c2', border: '#87e8de' }, // 青色
  { bg: '#fff7e6', text: '#faad14', border: '#ffd591' }, // 金黄色
  { bg: '#f0f5ff', text: '#2f54eb', border: '#adc6ff' }, // 深蓝
  { bg: '#f2f2f2', text: '#595959', border: '#d9d9d9' }, // 灰色
  { bg: '#fffbe6', text: '#d48806', border: '#ffe58f' }, // 琥珀
  { bg: '#e6fffb', text: '#08979c', border: '#87e8de' }, // 蓝绿
] as const;

// 使用简单的哈希函数为标签名生成索引
function getHashCode(str: string): number {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i);
    hash = ((hash << 5) - hash) + char;
    hash = hash & hash; // Convert to 32bit integer
  }
  return Math.abs(hash);
}

/**
 * 获取标签的颜色配置
 * 相同标签名始终返回相同颜色
 */
export function getTagColor(tagName: string) {
  const index = getHashCode(tagName) % COLOR_PALETTE.length;
  return COLOR_PALETTE[index];
}

/**
 * 生成标签的CSS样式对象
 */
export function getTagStyle(tagName: string) {
  const color = getTagColor(tagName);
  return {
    backgroundColor: color.bg,
    color: color.text,
    borderColor: color.border,
  };
}

/**
 * 生成标签的背景色（带透明度）
 * 用于标签在Item底部的显示
 */
export function getTagBackground(tagName: string): string {
  const color = getTagColor(tagName);
  // 将十六进制转换为rgba，添加透明度
  const hex = color.bg.replace('#', '');
  const r = parseInt(hex.slice(0, 2), 16);
  const g = parseInt(hex.slice(2, 4), 16);
  const b = parseInt(hex.slice(4, 6), 16);
  return `rgba(${r}, ${g}, ${b}, 0.85)`;
}

/**
 * 获取标签的文字颜色
 */
export function getTagTextColor(tagName: string): string {
  return getTagColor(tagName).text;
}
