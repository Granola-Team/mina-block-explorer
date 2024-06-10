suite(["@tier1"], "Not Found Page", () => {
  [
    {
      url: `/commands/GggGXNjmeiA59Kn1qiyG3NZ1oT1sBNBg8iwvLzJuyT7GH9dVmGggg`,
      message: "Transaction Not Found :(",
    },
  ].forEach(({ url, message }) =>
    it(`displays on ${url}`, () => {
      cy.visit(url);
      cy.contains(message);
    }),
  );
});
