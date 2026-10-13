// Public profile only: visitors choose Message on X; nothing is sent by this site.
export function sponsorContact(config) {
  const handle = typeof config?.xHandle === 'string' ? config.xHandle.trim().replace(/^@/, '') : '';
  if (!/^[A-Za-z0-9_]{1,15}$/.test(handle)) return null;
  return { handle, url: `https://x.com/${handle}` };
}
