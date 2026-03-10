#pragma once

#include <plugify/assembly.hpp>
#include <plugify/callback.hpp>
#include <plugify/language_module.hpp>
#include <plugify/logger.hpp>
#include <plugify/assembly_loader.hpp>
#include <plugify/extension.hpp>

#include <module_export.h>

struct RustString {
	const char* p{};
	size_t n{};

	operator std::string_view() const { return {p, n};  }
	operator bool() const { return n > 0;  }
};

struct RustLocation {
	size_t line{};
	size_t column{};
	RustString file;
	RustString function;
	RustString module;

	operator plg::source_location() const { return plg::source_location(line, column, file, function, module);  }
};

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
		const std::shared_ptr<ILogger>& GetLogger() { return _logger; }

	private:
		std::unique_ptr<Provider> _provider;
		std::shared_ptr<ILogger> _logger;
		std::shared_ptr<IAssemblyLoader> _loader;
		std::vector<std::unique_ptr<AssemblyHolder>> _assemblies;

		static const std::array<void*, 123> _pluginApi;
	};

	extern RustLanguageModule g_golm;
}

extern "C" RUSTLM_EXPORT ILanguageModule* GetLanguageModule();
