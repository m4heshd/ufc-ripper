<template>
  <ModMsgBox
      vID="modSearchHelp"
      vIcon="video_search"
      vTitle="How to use the search feature"
  >
    <div class="search-help">
      <div class="search-help__intro">
        Searching for videos on the UFC Fight Pass has been very difficult since the beginning. UFC Ripper's search
        feature allows you to do more advanced and very accurate searches through the Fight Pass library than UFC
        Fight Pass.
      </div>
      <div class="search-help__methods-title">
        You can refine your search by using following methods:
      </div>
      <div class="search-help__methods">
        <ul>
          <li>
            Use the <b>"Search titles only"</b> feature from the configuration to constrain the search
            algorithm to only look inside event titles and not the tags or descriptions, which can be vague and can
            pollute search results
          </li>
          <li>
            Use advanced syntax such as <b>Double quotes (phrase query)</b> which will force the search algorithm to
            only include results that matches the exact phrase you quoted, with the exact same order of words that
            matches the exact. For an example, by searching <b>"UFC 1"</b> instead of just <b>UFC 1</b>, you'll
            actually be able to find the first UFC event in the results
          </li>
          <li>
            Another useful type of advanced syntax is the <b>Negative symbol (-)</b>, aka <b>Prohibit operator</b>.
            This will omit any results with words that you have prefixed with the negative symbol. For an example, if
            you search <b>"Johnson -Michael"</b>, the results would not contain any events with Michael Johnson in
            them, but only other fighters named Johnson
          </li>
        </ul>
      </div>
    </div>
  </ModMsgBox>
</template>

<script setup>
// Components
import ModMsgBox from '@/components/ModMsgBox.vue';
</script>

<style lang="scss">
#modSearchHelp {
  max-width: 600rem;

  .search-help {
    display: flex;
    flex-direction: column;
    gap: 20rem;

    &__methods {
      &-title {
        font-size: 17rem;
        font-weight: bold;
        color: var(--primary);
      }

      & > ul > li {
        display: list-item;
        margin-left: 35rem;

        &::marker {
          font-size: 19rem;
        }

        & + li {
          margin-top: 15rem;
        }

        & > b {
          color: var(--primary);
        }
      }
    }
  }
}
</style>