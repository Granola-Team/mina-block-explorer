export const extractRows = (input) => {
  const regex = /Showing 1 to (\d+) of (\d+) records/;
  const match = input.match(regex);
  return match ? match[1] : null;
};
