import Cookies from 'js-cookie';

const TokenKey = 'Authori-zation';

export function getToken(): string | undefined {
  return Cookies.get(TokenKey);
}

export function setToken(token: string): string | undefined {
  return Cookies.set(TokenKey, token);
}

export function removeToken(): void {
  Cookies.remove(TokenKey);
}
