const defaultTitle = 'TyAdmin';

export default function getPageTitle(pageTitle?: string): string {
  if (pageTitle) {
    return `${pageTitle} - ${defaultTitle}`;
  }
  return defaultTitle;
}
