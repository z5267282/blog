/**
 * To convert between blog raw title and URL.
 */

export const blogToURL = (rawTitle) => rawTitle.replace(" ", "-");

export const URLtoBlog = (url) => url.replace("-", " ");
