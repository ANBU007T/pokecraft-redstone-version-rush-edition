// A direct-sold text placement. No ad network, tracking cookie, or third-party script.
// Keep campaign null until creative, dates, rights, and price have been agreed.
import { sponsorContact } from './sponsor-contact.js';
const slot = document.getElementById('sponsor-slot');

function activeCampaign(campaign) {
  if (!campaign || typeof campaign !== 'object') return null;
  const { name, message, url, startDate, endDate } = campaign;
  if (![name, message, url, startDate, endDate].every(v => typeof v === 'string' && v.trim())) return null;
  if (name.length > 48 || message.length > 100) return null;
  const dates = [startDate, endDate];
  if (!dates.every(d => /^\d{4}-\d{2}-\d{2}$/.test(d) && Number.isFinite(Date.parse(d)) && new Date(d).toISOString().slice(0, 10) === d)) return null;
  const today = new Date().toISOString().slice(0, 10);
  if (startDate > endDate || today < startDate || today > endDate) return null;
  try {
    const link = new URL(url);
    if (link.protocol !== 'https:' || link.username || link.password) return null;
    return { name, message, url: link.href };
  } catch { return null; }
}

if (slot) {
  try {
    const response = await fetch('sponsor-config.json', { cache: 'no-store' });
    if (!response.ok) throw new Error('Sponsor configuration unavailable');
    const config = await response.json();
    const campaign = activeCampaign(config.campaign);
    const link = document.createElement('a');
    link.className = 'sponsor-link';
    const label = document.createElement('span');
    label.className = 'sponsor-label';
    const message = document.createElement('span');
    message.className = 'sponsor-message';
    const arrow = document.createElement('span');
    arrow.className = 'sponsor-arrow';
    arrow.setAttribute('aria-hidden', 'true');

    if (campaign) {
      slot.setAttribute('aria-label', 'Paid advertisement');
      label.textContent = 'Sponsored';
      const name = document.createElement('strong');
      name.textContent = campaign.name;
      message.append(name, document.createTextNode(' · ' + campaign.message));
      link.href = campaign.url;
      link.target = '_blank';
      link.rel = 'sponsored noopener noreferrer';
      link.setAttribute('aria-label', `Sponsored: ${campaign.name}. ${campaign.message}. Opens in a new tab.`);
      arrow.textContent = '↗';
    } else {
      const contact = sponsorContact(config);
      slot.setAttribute('aria-label', 'Sponsorship inquiries');
      label.textContent = 'Ad space';
      message.textContent = contact ? `Sponsor this game · DM @${contact.handle} on X` : 'Sponsor this game';
      link.href = contact ? contact.url : 'sponsor.html';
      link.target = '_blank';
      link.rel = 'noopener noreferrer';
      link.setAttribute('aria-label', contact ? `Sponsor this game. Open @${contact.handle} on X to send a direct message. Opens in a new tab.` : 'Sponsor this game. Opens in a new tab.');
      arrow.textContent = '↗';
    }
    link.append(label, message, arrow);
    slot.replaceChildren(link);
    slot.hidden = false;
    window.dispatchEvent(new Event('resize'));
    // Text can wrap after viewport or font changes, so keep canvas sizing in sync.
    new ResizeObserver(() => window.dispatchEvent(new Event('resize'))).observe(slot);
  } catch {
    // A failed sponsor request (e.g. offline standalone play) keeps the static fallback link visible
    slot.hidden = false;
  }
}
