import { domToPng } from "modern-screenshot";
import { jsPDF } from "jspdf";

export async function exportToPdf(
  elementId: string,
  filename: string = "estadisticas",
  title: string = "Reporte de Estadísticas",
) {
  const element = document.getElementById(elementId);
  if (!element) {
    throw new Error(`Element with id "${elementId}" not found`);
  }

  const originalBg = element.style.backgroundColor;
  const originalPadding = element.style.padding;

  element.style.backgroundColor = "#ffffff";
  element.style.padding = "24px";

  const titleHtml = `
    <div style="
      font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      font-size: 24px;
      font-weight: 700;
      color: #1a1a1a;
      margin-bottom: 8px;
      padding-bottom: 16px;
      border-bottom: 2px solid #e5e7eb;
    ">${title}</div>
  `;

  const tempWrapper = document.createElement("div");
  tempWrapper.innerHTML = titleHtml;
  tempWrapper.style.marginBottom = "16px";

  element.insertBefore(tempWrapper, element.firstChild);

  try {
    const dataUrl = await domToPng(element, {
      backgroundColor: "#ffffff",
      scale: 2,
    });

    const contentWidth = element.offsetWidth;
    const contentHeight = element.offsetHeight;

    const pdf = new jsPDF({
      orientation: contentWidth > contentHeight ? "landscape" : "portrait",
      unit: "px",
      format: [contentWidth, contentHeight],
    });

    pdf.addImage(dataUrl, "PNG", 0, 0, contentWidth, contentHeight);
    pdf.save(`${filename}-${new Date().toISOString().split("T")[0]}.pdf`);
  } finally {
    element.removeChild(tempWrapper);
    element.style.backgroundColor = originalBg;
    element.style.padding = originalPadding;
  }
}