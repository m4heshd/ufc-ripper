<template>
  <ModMsgBox
      class="mod-support"
      vID="modSupport"
      vIcon="favorite"
      vTitle="Support this project"
  >
    <div class="mod-support__content">
      <div class="about">
        <p>
          <b>"UFC Ripper"</b> is a free, open-source tool that allows users with a paid subscription to access UFC Fight
          Pass content offline, in the format of their choice. Ideally, such services should offer these features
          without the need for third-party tools. However, since the service provider does not have these features built
          into their platform, the tool was developed to fulfill this need in a completely legal manner.
        </p>
        <p>
          It's worth mentioning that it was no easy task creating this tool. It required figuring out how UFC Fight Pass
          API backends work, countless hours of testing, and continuous maintenance for several months to reach this
          point. As the developer of this project, I do not receive any gain from it other than solely being satisfied
          by seeing people use this tool and appreciate it. Therefore, a donation would be greatly appreciated to
          support me in this endeavor.
        </p>
      </div>

      <div class="donation-sources">
        <div class="donation-sources__title">
          <b>You can send a donation my way using these methods:</b>
        </div>

        <div class="donation-sources__links center-content">
          <VAnchor href="https://ko-fi.variatix.net/">
            <img
                alt="Ko-fi support button"
                title="Support me on Ko-fi"
                src="@/assets/images/ko-fi.png"
            >
          </VAnchor>
          <VAnchor href="https://paypal.variatix.net/">
            <img
                alt="Paypal support button"
                title="Support me on Paypal"
                src="@/assets/images/paypal.png"
            >
          </VAnchor>
        </div>
      </div>

      <div class="donation-sources__github">
        <iframe
            src="https://github.com/sponsors/m4heshd/card"
            title="Sponsor m4heshd"
        ></iframe>
      </div>
    </div>
  </ModMsgBox>
</template>

<script setup>
// Components
import VAnchor from '@/components/VAnchor.vue';
import ModMsgBox from '@/components/ModMsgBox.vue';
</script>

<style lang="scss">
.mod-support {
  width: 100%;
  max-width: 600rem;
  max-height: 80%;
  display: grid;
  grid-template-rows: max-content minmax(0rem, 1fr) max-content;
  overflow: hidden;

  &__content {
    overflow-y: auto;

    .about {
      & > p {
        text-indent: 50rem;
        margin-bottom: 20rem;
      }
    }

    .donation-sources {
      &__title {
        margin-bottom: 20rem;
        font-size: 16rem;
        color: var(--primary);
      }

      &__links {
        margin-bottom: 20rem;
        gap: 20rem;

        & > a > img {
          border-radius: 0;
        }
      }

      &__github {
        border-radius: 30rem;

        & > iframe {
          width: 100%;
          height: 180rem;
          border: 0;
        }
      }
    }
  }
}
</style>
