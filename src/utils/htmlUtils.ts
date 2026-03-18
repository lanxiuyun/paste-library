/**
 * HTML 工具函数
 * 用于处理 HTML 实体解码和标签剥离
 */

/**
 * 解码 HTML 实体
 * 如: &#160; -> (空格), &amp; -> &, &lt; -> <, &gt; -> >
 * 
 * @param html 包含 HTML 实体的字符串
 * @returns 解码后的字符串
 */
export function decodeHtmlEntities(html: string): string {
  if (!html) return '';
  
  // 使用 textarea 的 innerHTML 特性解码 HTML 实体
  const textarea = document.createElement('textarea');
  textarea.innerHTML = html;
  let decoded = textarea.value;
  
  // 将不间断空格 (U+00A0, &nbsp;) 转换为普通空格 (U+0020)
  // 这样避免在某些编辑器中显示为乱码或方框
  decoded = decoded.replace(/\u00A0/g, ' ');
  
  return decoded;
}

/**
 * 去除 HTML 标签
 * 
 * @param html HTML 字符串
 * @returns 去除标签后的纯文本
 */
export function stripHtmlTags(html: string): string {
  if (!html) return '';
  return html.replace(/<[^>]*>/g, '');
}

/**
 * 去除 HTML 标签并解码实体
 * 这是一个组合函数，用于将 HTML 内容转换为干净的纯文本
 * 在去除块级元素标签时添加换行符以保持格式
 * 
 * @param html HTML 字符串
 * @returns 去除标签并解码实体后的纯文本
 */
export function stripHtmlAndDecode(html: string): string {
  if (!html) return '';

  let text = html;

  // 先将 <br> 标签转换为换行符
  text = text.replace(/<br\s*\/?>/gi, '\n');

  // 处理块级元素：在 <div>, <p> 等标签后添加换行
  // 替换闭合标签为换行符
  text = text.replace(/<\/(?:div|p|li|tr|h[1-6])>/gi, '\n');
  // 替换开始标签为空字符串（保留内容）
  text = text.replace(/<(div|p|li|tr|h[1-6])[^>]*>/gi, '');

  // 解码 HTML 实体（包括 &#160; 等）
  text = decodeHtmlEntities(text);

  // 去除所有剩余的 HTML 标签（如 <span>, <a> 等行内元素）
  text = stripHtmlTags(text);

  // 清理多余的空白行（最多保留一个空行）
  text = text.replace(/\n{3,}/g, '\n\n');

  return text.trim();
}

/**
 * 将 HTML 转换为格式化的纯文本
 * 保留换行格式（将 <br>, <p> 等标签转换为换行符）
 * 用于预览显示
 * 
 * @param html HTML 字符串
 * @returns 格式化的纯文本
 */
export function htmlToFormattedText(html: string): string {
  if (!html) return '';

  // 第一步：处理标签内的换行（在解码实体之前）
  let text = html;

  // 处理 <br> 标签（包括自闭合形式）
  text = text.replace(/<br\s*\/?>/gi, '\n');

  // 处理 <p> 标签 - 段落间添加空行
  text = text.replace(/<p[^>]*>(.*?)<\/p>/gi, '\n$1\n');

  // 处理 <div> 标签 - 块级元素后添加换行
  text = text.replace(/<div[^>]*>(.*?)<\/div>/gi, '\n$1\n');

  // 处理其他常见的块级元素
  text = text.replace(/<\/(?:li|tr|h[1-6])>/gi, '\n');
  text = text.replace(/<(?:li|tr)[^>]*>/gi, '');

  // 去除所有剩余的 HTML 标签
  text = stripHtmlTags(text);

  // 第二步：解码 HTML 实体（包括将不间断空格转为普通空格）
  text = decodeHtmlEntities(text);

  // 第三步：清理格式
  // 合并多个连续换行为最多两个
  text = text.replace(/\n{3,}/g, '\n\n');

  // 清理每行首尾的空白，但保留空行
  return text.split('\n')
    .map(line => line.trim())
    .join('\n')
    .trim();
}

/**
 * 将 HTML 转换为纯文本，用于写入剪贴板
 * 仅将 <br> 标签转换为换行符，不添加额外格式
 * 
 * @param html HTML 字符串
 * @returns 纯文本，保留原有换行结构
 */
export function htmlToClipboardText(html: string): string {
  if (!html) return '';

  let text = html;

  // 只处理 <br> 标签为换行符（这是代码中显式的换行）
  text = text.replace(/<br\s*\/?>/gi, '\n');

  // 处理 <pre> 标签内容 - 保持原样，只是去除标签
  text = text.replace(/<pre[^>]*>([\s\S]*?)<\/pre>/gi, '\n$1\n');

  // 去除所有剩余的 HTML 标签（包括 <div>, <span> 等，但保留其中的文本）
  text = stripHtmlTags(text);

  // 解码 HTML 实体
  text = decodeHtmlEntities(text);

  // 清理多余的空白行（最多保留一个空行）
  text = text.replace(/\n{3,}/g, '\n\n');

  return text.trim();
}
