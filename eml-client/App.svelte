<script>
  import Icon from "fa-svelte";
  import { createEventDispatcher } from "svelte";
  import { faEdit } from "@fortawesome/free-solid-svg-icons/faEdit";
  import { faTag } from "@fortawesome/free-solid-svg-icons/faTag";
  import {
    Badge,
    Button,
    Col,
    Container,
    Nav,
    NavItem,
    Navbar,
    NavbarBrand,
    NavLink,
    Row,
  } from "sveltestrap";

  import * as api from "./api.js";
  import EmlList from "./components/EmlList.svelte";
  import EmlNewModal from "./components/EmlNewModal.svelte";
  import Eml from "./components/Eml.svelte";
  import NavQueryItem from "./components/NavQueryItem.svelte";
  import Search from "./components/Search.svelte";
  import TagsSelect from "./components/TagsSelect.svelte";
  import TagQueryModal from "./components/TagQueryModal.svelte";

  const dispatch = createEventDispatcher();

  let emlSelected = null;
  let specialQueries = [];
  let tagQueries = [];
  let querySelected = "tag:inbox and not tag:spam";

  let newEmlModalOpen = false;

  let tagModalOpen = false;
  let tagSelected;

  const markRead = (id) => api.rmTag(`id:${id}`, "unread").then(dispatch);

  const refreshTagList = () => {
    api.listTags().then((tagList) => {
      const allTagQueries = tagList.map((t) => {
        let query = ["inbox", "sent"].includes(t)
          ? `tag:${t}`
          : `tag:${t} and tag:inbox`;

        if (t != "spam") query += " and not tag:spam";

        api.countMatches(`(${query}) and tag:unread`).then((c) => {
          const tIdx = tagQueries.findIndex((e) => e.name == t);
          if (tIdx != -1) tagQueries[tIdx].unreadCount = c;
        });

        api.countMatches(query).then((c) => {
          const tIdx = tagQueries.findIndex((e) => e.name == t);
          if (tIdx != -1) tagQueries[tIdx].totalCount = c;
        });

        return Object({
          name: t,
          query,
          totalCount: (tagQueries.find((e) => e.name == t) ?? {}).totalCount,
          unreadCount: (tagQueries.find((e) => e.name == t) ?? {}).unreadCount,
        });
      });

      const specials = ["inbox", "unread", "sent", "spam"]; // ordered
      specialQueries = specials.map((n) =>
        allTagQueries.find((e) => e.name == n)
      );
      tagQueries = allTagQueries.filter((t) => !specials.includes(t.name));
    });

    const arraySetEqual = (a, b) =>
      a.length == b.length && a.every((e) => b.indexOf(e) != -1);

    if (emlSelected)
      api.listEml(`id:${emlSelected.id}`).then(([emlMeta]) => {
        // update if different only to avoid recursion
        if (!arraySetEqual(emlSelected.tags, emlMeta.Ok.tags))
          emlSelected.tags = emlMeta.Ok.tags;
      });
  };

  const refreshQuery = () => (querySelected = new String(querySelected));

  refreshTagList();

  $: if (emlSelected != null) {
    markRead(emlSelected.id).then(refreshQuery).then(refreshTagList);
  }
</script>

<Container fluid class="h-100 d-flex flex-column">
  <Row class="flex-shrink-0">
    <Navbar color="primary">
      <NavbarBrand><h1>Amail</h1></NavbarBrand>
      <Button on:click={() => (newEmlModalOpen = true)}>
        <Icon icon={faEdit} />
      </Button>
      <EmlNewModal bind:isOpen={newEmlModalOpen} />
    </Navbar>
  </Row>

  <Row class="flex-fill" style="min-height: 0;">
    <Col xs="1" class="border mh-100 scroll">
      <Nav vertical pills>
        {#each specialQueries as tag}
          <NavQueryItem
            name={tag.name}
            unreadCount={tag.unreadCount}
            selected={tag.query == querySelected}
            on:select={() => (querySelected = tag.query)}
          />
        {/each}

        <hr />

        {#each tagQueries.filter((t) => t.totalCount > 0) as tag}
          <NavQueryItem
            name={tag.name}
            unreadCount={tag.unreadCount}
            selected={tag.query == querySelected}
            on:select={() => (querySelected = tag.query)}
          />
        {/each}
      </Nav>
    </Col>

    <Col xs="4" class="border-bottom h-100 d-flex flex-column">
      <Row style="margin: 0.3rem">
        <Col>
          <Search
            bind:querySelected
            quietQueries={specialQueries.concat(tagQueries).map((t) => t.query)}
          />
        </Col>

        <Col xs="2">
          <TagsSelect
            on:tagSelected={(tag) => {
              tagSelected = tag.detail;
              tagModalOpen = true;
            }}
          />
        </Col>

        <TagQueryModal
          bind:isOpen={tagModalOpen}
          tag={tagSelected}
          query={querySelected}
          on:tagsUpdated={() => {
            refreshTagList();
            refreshQuery();
          }}
        />
      </Row>

      <Row class="flex-fill mh-100 scroll">
        <EmlList
          query={querySelected}
          bind:emlSelected
          hideTags={new Set([
            (tagQueries.find((t) => t.query == querySelected) ?? {}).name,
          ])}
        />
      </Row>
    </Col>

    {#if emlSelected}
      <Col class="border h-100 d-flex flex-column" style="min-width: 0;">
        <Eml
          emlMeta={emlSelected}
          on:tagsUpdated={() => {
            refreshTagList();
            refreshQuery();
          }}
        />
      </Col>
    {:else}
      <Col class="bg-light" />
    {/if}
  </Row>

  <Row class="flex-shrink-0">
    <Col class="mh-100" style="min-height: 1rem;" />
  </Row>
</Container>
