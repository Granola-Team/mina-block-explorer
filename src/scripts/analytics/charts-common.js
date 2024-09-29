function getBlockLimit() {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  return urlParams.get("limit") || 1000;
}
