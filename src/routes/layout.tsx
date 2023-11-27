import { component$, Slot } from "@builder.io/qwik";
import type { RequestHandler, DocumentHead } from "@builder.io/qwik-city";
import Nav from "~/components/navigation/nav";

export const onGet: RequestHandler = async ({ cacheControl }) => {
  cacheControl({
    staleWhileRevalidate: 60 * 60 * 24 * 7,
    maxAge: 5,
  });
};

export const head: DocumentHead = {
  title: "Mina Block Explorer",
  meta: [
    {
      name: "description",
      content: "For exploring MINA blockchain",
    },
  ],
};

export default component$(() => {
  return (
    <>
      <Nav />
      <main>
        <Slot />
      </main>
    </>
  );
});
