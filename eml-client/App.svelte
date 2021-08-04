<script>
  import Icon from "fa-svelte";
  import { createEventDispatcher } from "svelte";
  import { faTag } from "@fortawesome/free-solid-svg-icons/faTag";
  import {
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
  import Eml from "./components/Eml.svelte";
  import Search from "./components/Search.svelte";
  import TagsSelect from "./components/TagsSelect.svelte";
  import TagQueryModal from "./components/TagQueryModal.svelte";

  const dispatch = createEventDispatcher();

  let emlSelected = null;
  let tagQueries = [];
  let querySelected = "tag:inbox";

  let tagModalOpen = false;
  let tagSelected;

  const markRead = (id) => api.rmTag(`id:${id}`, "unread").then(dispatch);

  const refreshTagList = () =>
    api.listTags().then((tagList) => {
      tagQueries = tagList.map((t) => `tag:${t}`);
    });

  const refreshQuery = () => (querySelected = new String(querySelected));

  refreshTagList();

  $: if (emlSelected != null) {
    markRead(emlSelected.id).then(refreshQuery);
  }
</script>

<Container fluid class="h-100 d-flex flex-column">
  <Row class="flex-shrink-0">
    <Navbar color="primary">
      <NavbarBrand><h1>Amail</h1></NavbarBrand>
    </Navbar>
  </Row>

  <Row class="flex-fill" style="min-height: 0;">
    <Col xs="1" class="border mh-100 scroll">
      <Nav vertical pills>
        {#each tagQueries as tag}
          <NavItem>
            <NavLink
              active={tag == querySelected}
              on:click={() => (querySelected = tag)}
            >
              <Icon icon={faTag} />
              <h2><span>{tag.split("tag:")[1]}</span></h2>
            </NavLink>
          </NavItem>
        {/each}
      </Nav>
    </Col>

    <Col xs="4" class="border-bottom h-100 d-flex flex-column">
      <Row style="margin: 0.3rem">
        <Col>
          <Search bind:querySelected quietQueries={tagQueries} />
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
          hideTags={new Set([querySelected.split("tag:")[1]])}
        />
      </Row>
    </Col>

    {#if emlSelected}
      <Col class="border h-100 d-flex flex-column" style="min-width: 0;">
        <Eml emlMeta={emlSelected} />
      </Col>
    {:else}
      <Col class="bg-light" />
    {/if}
  </Row>

  <Row class="flex-shrink-0">
    <Col class="mh-100" style="min-height: 1rem;" />
  </Row>
</Container>

<style scoped>
</style>
