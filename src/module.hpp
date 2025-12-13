#pragma once

#include <plugify/assembly.hpp>
#include <plugify/callback.hpp>
#include <plugify/language_module.hpp>
#include <plugify/extension.hpp>

#include <module_export.h>

using namespace plugify;

namespace rustlm {
	constexpr int kApiVersion = 1;

	struct PluginContext {
		bool hasUpdate{};
		bool hasStart{};
		bool hasEnd{};
	};

	using MainFunc = void (*)();
	using InitFunc = int (*)(void* const*, size_t, int, const void*);
	using StartFunc = void (*)();
	using UpdateFunc = void (*)(float);
	using EndFunc = void (*)();
	using ContextFunc = PluginContext* (*)();

	struct AssemblyHolder {
		std::shared_ptr<IAssembly> assembly;
		UpdateFunc updateFunc;
		StartFunc startFunc;
		EndFunc endFunc;
		ContextFunc contextFunc;
	};

	class RustLanguageModule final : public ILanguageModule {
	public:
		RustLanguageModule() = default;
		~RustLanguageModule() = default;

		// ILanguageModule
		Result<InitData> Initialize(const Provider& provider, const Extension& module) override;
		void Shutdown() override;
		void OnUpdate(std::chrono::milliseconds dt) override;
		Result<LoadData> OnPluginLoad(const Extension& plugin) override;
		void OnPluginStart(const Extension& plugin) override;
		void OnPluginUpdate(const Extension& plugin, std::chrono::milliseconds dt) override;
		void OnPluginEnd(const Extension& plugin) override;
		void OnMethodExport(const Extension& plugin) override;
		bool IsDebugBuild() override;

		const std::unique_ptr<Provider>& GetProvider() { return _provider; }

	private:
		std::unique_ptr<Provider> _provider;
		std::vector<std::unique_ptr<AssemblyHolder>> _assemblies;

		static const std::array<void*, 122> _pluginApi;
	};

	extern RustLanguageModule g_golm;
}

extern "C" RUSTLM_EXPORT ILanguageModule* GetLanguageModule();
