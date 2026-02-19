export function isExternal(path: string): boolean {
  return /^(https?:|mailto:|tel:)/.test(path);
}

export function isValidURL(url: string): boolean {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
}

export function isPhone(val: string): boolean {
  return /^1[3-9]\d{9}$/.test(val);
}

export function isEmail(val: string): boolean {
  return /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(val);
}
